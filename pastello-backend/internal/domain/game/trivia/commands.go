package trivia

// Cmd is the domain command union consumed by the Trivia engine.
type Cmd struct {
	SubmitAnswer *SubmitAnswer
	RevealHint   *RevealHint
}

type SubmitAnswer struct {
	PlayerID string
	Answer   string
}

type RevealHint struct{}
