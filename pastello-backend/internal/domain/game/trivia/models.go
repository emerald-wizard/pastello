package trivia

// Minimal state scaffold; extend as rules evolve.
type State struct {
	Scores map[string]int // keyed by PlayerID string
	Hints  []string
}

func NewState() State {
	return State{Scores: map[string]int{}, Hints: nil}
}
