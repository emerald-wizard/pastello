.PHONY: proto.lint proto.gen run fe

proto.lint:
	buf lint

proto.gen:
	buf generate

run:
	go run ./cmd/server

fe:
	cd frontend && pnpm i &&