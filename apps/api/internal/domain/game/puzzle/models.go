package puzzle

import "fmt"

type Pos struct{ X, Y int }

type Move struct {
	From Pos
	To   Pos
}

type State struct {
	Width, Height int
	Tiles         map[string]struct{}
	History       []Move
}

func NewState(w, h int) State {
	return State{
		Width: w, Height: h,
		Tiles: make(map[string]struct{}),
	}
}

func key(x, y int) string { return fmt.Sprintf("%d,%d", x, y) }
