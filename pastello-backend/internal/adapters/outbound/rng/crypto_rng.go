package rng

import (
	"crypto/rand"
	"encoding/hex"

	out "github.com/runecraft-studios/pastello/internal/ports"
)

// IDGen returns a 128-bit cryptographically-random hex string (32 chars).
// Stable format, no external deps, good for session IDs, etc.
type IDGen struct{}

func (IDGen) New() string {
	var b [16]byte
	_, _ = rand.Read(b[:]) // best-effort; failure is extremely unlikely
	return hex.EncodeToString(b[:])
}

var _ out.IDGen = (IDGen{})
