package ws

import (
	"github.com/runecraft-studios/pastello/internal/application/commands"
	web "github.com/runecraft-studios/pastello/internal/gen/runecraftstudios/pastello/web/game/v1"
)

// Envelope (protobuf) -> app command DTO (no protobuf outside adapters)
func toAppCmd(env *web.Envelope) any {
	switch m := env.GetBody().(type) {
	case *web.Envelope_PuzzleMovePiece:
		c := m.PuzzleMovePiece
		return commands.PuzzleMovePiece{
			FromX: int(c.FromX), FromY: int(c.FromY),
			ToX: int(c.ToX), ToY: int(c.ToY),
		}
	case *web.Envelope_PuzzleUndoMove:
		return commands.PuzzleUndoMove{}
	case *web.Envelope_TriviaSubmitAnswer:
		c := m.TriviaSubmitAnswer
		return commands.TriviaSubmitAnswer{PlayerID: c.GetPlayerId().GetValue(), Answer: c.GetAnswer()}
	case *web.Envelope_TriviaRevealHint:
		return commands.TriviaRevealHint{}
	}
	return nil
}

// Pull the session id from whichever command it is.
func extractSessionID(env *web.Envelope) string {
	switch m := env.GetBody().(type) {
	case *web.Envelope_PuzzleMovePiece:
		return m.PuzzleMovePiece.GetSessionId().GetValue()
	case *web.Envelope_PuzzleUndoMove:
		return m.PuzzleUndoMove.GetSessionId().GetValue()
	case *web.Envelope_TriviaSubmitAnswer:
		return m.TriviaSubmitAnswer.GetSessionId().GetValue()
	case *web.Envelope_TriviaRevealHint:
		return m.TriviaRevealHint.GetSessionId().GetValue()
	default:
		return ""
	}
}

// App/domain payload -> Envelope (protobuf), echo correlation id
func toEdgeEvent(corr string, payload any, sessionID string) *web.Envelope {
	switch p := payload.(type) {
	case struct{ FromX, FromY, ToX, ToY int }:
		return envPuzzlePieceMoved(corr, sessionID, int32(p.FromX), int32(p.FromY), int32(p.ToX), int32(p.ToY))
	case struct{}:
		return envPuzzleMoveUndone(corr, sessionID)
	case struct{ Delta, Total int }:
		return envTriviaAnswerAccepted(corr, sessionID, int32(p.Delta), int32(p.Total))
	case struct{ Hint string }:
		return envTriviaHintRevealed(corr, sessionID, p.Hint)
	default:
		return &web.Envelope{CorrelationId: corr}
	}
}
