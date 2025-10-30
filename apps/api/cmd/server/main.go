package main

import (
	"log"
	"net/http"
	"time"

	"github.com/runecraft-studios/pastello/apps/api/internal/adapters/inbound/ws"
	"github.com/runecraft-studios/pastello/apps/api/internal/adapters/outbound/eventbus"
	"github.com/runecraft-studios/pastello/apps/api/internal/adapters/outbound/memory"
	"github.com/runecraft-studios/pastello/apps/api/internal/adapters/outbound/rng"
	"github.com/runecraft-studios/pastello/apps/api/internal/application/services"
)

type clockNow struct{}

func (clockNow) Now() time.Time { return time.Now() }

func main() {
	// Outbound adapters
	repo := memory.NewSessionRepo()
	bus := eventbus.NopBus{}
	idg := rng.IDGen{} // your ID generator (implements ports.IDGen)
	clk := clockNow{}  // simple system clock (implements ports.Clock)

	// Engine factory (inject ports). RNG is optional; set if you have one.
	engFactory := services.EngineFactoryImpl{
		Clock: clk,
		// RNG:   your RNG impl, if any
		IDGen: idg,
	}

	// Compose Service façade from individual use cases
	gameSvc := services.NewGameService(services.Deps{
		Repo:  repo,
		Bus:   bus,
		Clock: clk,
		IDGen: idg,
		Engs:  engFactory, // <— services.EngineFactoryImpl
	})

	// Command registry (thin dispatcher) depends on the façade
	reg := services.CommandRegistry{Svc: gameSvc}

	// WS router + server
	router := ws.NewRouter(
		ws.GameStartHandler{Svc: gameSvc},
		ws.GameCommandHandler{Registry: reg},
		ws.NoopAuth{}, // or ws.BearerAuth{AcceptToken: "dev-token", Subject: "dev-user"}
	)
	server := ws.NewServer(router)

	http.HandleFunc("/ws", server.HandleWS)
	log.Println("listening on :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
