package eventbus

import (
	"context"
	"log"

	"github.com/runecraft-studios/pastello/internal/ports"
)

// NopBus publishes nothing. Handy for dev/tests.
type NopBus struct {
	Logger *log.Logger // optional; if set, we log topic + type
}

func (b NopBus) Publish(ctx context.Context, topic string, payload any) error {
	_ = ctx
	if b.Logger != nil {
		b.Logger.Printf("eventbus(nop): topic=%s payload=%T", topic, payload)
	}
	return nil
}

var _ ports.EventBus = NopBus{}
