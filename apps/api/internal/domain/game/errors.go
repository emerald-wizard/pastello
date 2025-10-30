package game

import "errors"

var (
	ErrWrongEngine        = errors.New("wrong engine for command")
	ErrUnsupportedCommand = errors.New("unsupported command")
)
