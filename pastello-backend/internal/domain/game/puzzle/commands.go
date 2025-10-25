package puzzle

// Cmd is the domain command union that the engine consumes.
type Cmd struct {
	MovePiece *MovePiece
	UndoMove  *UndoMove
}

type MovePiece struct{ FromX, FromY, ToX, ToY int }
type UndoMove struct{}
