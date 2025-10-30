cd internal/application/ports/inbound/websocket/
protoc --go_out=./ --go_opt=paths=source_relative \
  ./proto/session.proto