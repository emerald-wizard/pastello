module github.com/runecraft-studios/pastello/apps/api

go 1.25.3

replace github.com/runecraft-studios/pastello/packages/contracts/gen/go => ../../packages/contracts/gen/go

require (
	github.com/gorilla/websocket v1.5.3
	github.com/runecraft-studios/pastello/packages/contracts/gen/go v0.0.0-00010101000000-000000000000
	google.golang.org/protobuf v1.36.10
	gopkg.in/yaml.v2 v2.4.0
)
