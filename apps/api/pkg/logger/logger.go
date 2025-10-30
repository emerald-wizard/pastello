package logger

import (
	"fmt"
	"strings"
)

// Logger is a simple logging interface.
type Logger interface {
	Info(msg string)
	Error(msg string)
	Sync() error
}

type simpleLogger struct {
	level string
}

func NewLogger(level string) Logger {
	return &simpleLogger{level: strings.ToLower(level)}
}

func (l *simpleLogger) Info(msg string) {
	if l.level == "info" || l.level == "debug" {
		fmt.Println("[INFO]", msg)
	}
}

func (l *simpleLogger) Error(msg string) {
	fmt.Println("[ERROR]", msg)
}

func (l *simpleLogger) Sync() error {
	// No-op for this simple logger
	return nil
}
