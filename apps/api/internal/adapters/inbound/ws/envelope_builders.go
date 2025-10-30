package ws

import (
	puzzlev1 "github.com/runecraft-studios/pastello/packages/contracts/gen/go/runecraftstudios/pastello/game/puzzle/v1"
	triviav1 "github.com/runecraft-studios/pastello/packages/contracts/gen/go/runecraftstudios/pastello/game/trivia/v1"
	typesv1 "github.com/runecraft-studios/pastello/packages/contracts/gen/go/runecraftstudios/pastello/game/types/v1"
	web "github.com/runecraft-studios/pastello/packages/contracts/gen/go/runecraftstudios/pastello/web/game/v1"
)

// Each helper returns a *web.Envelope with the correct oneof wrapper.
// No attempt to use IsEnvelope_Body (it’s unexported); be explicit.

func envPuzzlePieceMoved(corr, sessionID string, fromX, fromY, toX, toY int32) *web.Envelope {
	return &web.Envelope{
		CorrelationId: corr,
		Body: &web.Envelope_PuzzlePieceMoved{
			PuzzlePieceMoved: &puzzlev1.PieceMovedEvent{
				SessionId: &typesv1.GameSessionId{Value: sessionID},
				FromX:     fromX, FromY: fromY, ToX: toX, ToY: toY,
			},
		},
	}
}

func envPuzzleMoveUndone(corr, sessionID string) *web.Envelope {
	return &web.Envelope{
		CorrelationId: corr,
		Body: &web.Envelope_PuzzleMoveUndone{
			PuzzleMoveUndone: &puzzlev1.MoveUndoneEvent{
				SessionId: &typesv1.GameSessionId{Value: sessionID},
			},
		},
	}
}

func envTriviaAnswerAccepted(corr, sessionID string, delta, total int32) *web.Envelope {
	return &web.Envelope{
		CorrelationId: corr,
		Body: &web.Envelope_TriviaAnswerAccepted{
			TriviaAnswerAccepted: &triviav1.AnswerAcceptedEvent{
				SessionId:  &typesv1.GameSessionId{Value: sessionID},
				DeltaScore: delta,
				TotalScore: total,
			},
		},
	}
}

func envTriviaHintRevealed(corr, sessionID, hint string) *web.Envelope {
	return &web.Envelope{
		CorrelationId: corr,
		Body: &web.Envelope_TriviaHintRevealed{
			TriviaHintRevealed: &triviav1.HintRevealedEvent{
				SessionId: &typesv1.GameSessionId{Value: sessionID},
				HintText:  hint,
			},
		},
	}
}

func envError(corr, code, msg string) *web.Envelope {
	return &web.Envelope{
		CorrelationId: corr,
		Body: &web.Envelope_Error{
			Error: &web.ErrorEvent{Code: code, Message: msg},
		},
	}
}
