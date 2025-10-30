package ports

import "context"

// EventBus publishes domain/application events to external systems.
// Implementations may be Kafka/NATS/PubSub/etc. Start with a NOP for dev.
type EventBus interface {
	Publish(ctx context.Context, topic string, payload any) error
}
