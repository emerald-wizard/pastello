package ports

// IDGen produces unique IDs for sessions, commands, etc.
type IDGen interface {
	New() string
}
