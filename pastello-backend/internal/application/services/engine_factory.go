package services

//Picks a concrete engine for a session’s GameType and injects ports (Clock/RNG/ID).

import (
	"github.com/runecraft-studios/pastello/internal/domain/game"
	"github.com/runecraft-studios/pastello/internal/domain/game/puzzle"
	"github.com/runecraft-studios/pastello/internal/domain/game/trivia"
	"github.com/runecraft-studios/pastello/internal/ports"
)

type EngineFactory interface {
	For(t game.GameType) game.Engine
}

type EngineFactoryImpl struct {
	Clock ports.Clock
	RNG   ports.RNG
	IDGen ports.IDGen
}

func (f EngineFactoryImpl) For(t game.GameType) game.Engine {
	switch t {
	case game.GameTypePuzzle:
		return puzzle.NewEngine(puzzle.Deps{Clock: f.Clock, RNG: f.RNG, IDGen: f.IDGen})
	case game.GameTypeTrivia:
		return trivia.NewEngine(trivia.Deps{Clock: f.Clock, RNG: f.RNG, IDGen: f.IDGen})
	default:
		return nil
	}
}
