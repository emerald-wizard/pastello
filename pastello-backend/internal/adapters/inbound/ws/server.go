package ws

import (
	"context"
	"errors"
	"net/http"
	"sync"
	"time"

	"github.com/gorilla/websocket"
	"google.golang.org/protobuf/proto"

	web "github.com/runecraft-studios/pastello/internal/gen/runecraftstudios/pastello/web/game/v1"
)

type Server struct {
	router   *Router
	upgrader websocket.Upgrader
	writeMu  sync.Mutex // synchronize writes
}

func NewServer(r *Router) *Server {
	return &Server{
		router: r,
		upgrader: websocket.Upgrader{
			ReadBufferSize:  1024,
			WriteBufferSize: 1024,
			// Lock this down in prod to your allowed origins
			CheckOrigin: func(r *http.Request) bool {
				origin := r.Header.Get("Origin")
				return origin == "http://localhost:5173" ||
					origin == "https://pastello.example" ||
					origin == ""
			},
		},
	}
}

func (s *Server) HandleWS(w http.ResponseWriter, r *http.Request) {
	// 1) Auth before upgrade (clean 401 path)
	baseCtx, err := s.router.Auth().OnUpgrade(r)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	// --- Upgrade ---
	conn, err := s.upgrader.Upgrade(w, r, nil)
	if err != nil {
		http.Error(w, "upgrade failed", http.StatusBadRequest)
		return
	}
	defer conn.Close()

	// Per-connection context derived from auth context (cancellable)
	ctx, cancel := context.WithCancel(baseCtx)
	defer cancel()

	// Reasonable read limits + compression
	conn.SetReadLimit(1 << 20) // 1MB
	conn.EnableWriteCompression(true)

	// Read deadline + pong handler (keeps the connection fresh)
	_ = conn.SetReadDeadline(time.Now().Add(60 * time.Second))
	conn.SetPongHandler(func(string) error {
		return conn.SetReadDeadline(time.Now().Add(60 * time.Second))
	})

	// Heartbeat: periodic ping
	pingStop := make(chan struct{})
	go s.pingLoop(conn, pingStop)

	// --- Main read loop: one reader, synchronized writers ---
	for {
		mt, data, err := conn.ReadMessage()
		if err != nil {
			break // client closed or network error
		}
		if mt != websocket.BinaryMessage {
			continue // ignore text/other frames
		}

		var env web.Envelope
		if err := proto.Unmarshal(data, &env); err != nil {
			_ = s.sendError(conn, "BAD_PROTOBUF", "could not decode envelope")
			continue
		}

		// Optional per-envelope authorization/policy
		if s.router.auth != nil {
			if newCtx, err := s.router.auth.OnEnvelope(ctx, &env); err != nil {
				_ = s.sendError(conn, "AUTH_ERROR", err.Error())
				continue
			} else if newCtx != nil {
				ctx = newCtx
			}
		}

		response, err := s.router.Route(ctx, &env)
		if err != nil {
			_ = s.sendError(conn, "ROUTING_ERROR", err.Error())
			continue
		}
		if response == nil {
			continue // no response for this message
		}

		if err := s.sendEnvelope(conn, response); err != nil {
			break // write failed; exit loop and close
		}
	}

	// Clean shutdown: send a close control frame with a reason
	closeMsg := websocket.FormatCloseMessage(websocket.CloseNormalClosure, "bye")
	_ = s.writeControl(conn, websocket.CloseMessage, closeMsg)
	close(pingStop)
}

func (s *Server) sendEnvelope(conn *websocket.Conn, out *web.Envelope) error {
	b, err := proto.Marshal(out)
	if err != nil {
		return err
	}
	return s.writeBinary(conn, b)
}

func (s *Server) sendError(conn *websocket.Conn, code, msg string) error {
	// TODO: if you add an ErrorEnvelope to your proto, build & send it here.
	return errors.New("no error envelope type defined yet")
}

// ---- Write helpers (synchronized, with deadlines) ----

func (s *Server) writeBinary(conn *websocket.Conn, b []byte) error {
	s.writeMu.Lock()
	defer s.writeMu.Unlock()
	_ = conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
	return conn.WriteMessage(websocket.BinaryMessage, b)
}

func (s *Server) writeControl(conn *websocket.Conn, mt int, data []byte) error {
	s.writeMu.Lock()
	defer s.writeMu.Unlock()
	deadline := time.Now().Add(2 * time.Second)
	return conn.WriteControl(mt, data, deadline)
}

// Periodic pings to keep idle connections alive and detect dead peers.
func (s *Server) pingLoop(conn *websocket.Conn, stop <-chan struct{}) {
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()
	for {
		select {
		case <-ticker.C:
			_ = s.writeControl(conn, websocket.PingMessage, nil)
		case <-stop:
			return
		}
	}
}
