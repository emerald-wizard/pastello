Here’s the tight recap of what we landed on for **Proto layout** and **why**.

# Final repo layout (sources vs generated)

```
repo-root/
  buf.yaml
  buf.gen.yaml

  proto/                                  ← all .proto **sources**
    runcraftstudios/
      pastello/
        game/
          types/v1/        # shared, cross-game enums/IDs, etc.
            types.proto
          session/v1/      # create/start session API
            session.proto
          trivia/v1/       # feature-specific contracts
            rules.proto
            commands.proto
            events.proto
          puzzle/v1/       # feature-specific contracts
            rules.proto
            commands.proto
            events.proto
        web/
          game/v1/         # edge envelopes/messages for WS/HTTP
            envelope.proto

  internal/
    gen/                   ← **Go generated code** (by Buf)
      runcraftstudios/
        pastello/
          game/...
          web/...

  frontend/
    src/proto/             ← **TypeScript generated code** (by Buf)
      runcraftstudios/
        pastello/
          game/...
          web/...
```

## Package lines (match folders exactly)

Examples at the top of each `.proto`:

```proto
syntax = "proto3";
package runcraftstudios.pastello.game.trivia.v1; // mirrors proto path
```

Same pattern for `...game.puzzle.v1`, `...game.session.v1`, `...game.types.v1`, and `...web.game.v1`.

# Buf configs we use

**`buf.yaml` (workspace at repo root)**

```yaml
version: v2
modules:
  - path: proto
lint:
  use: [DEFAULT]
breaking:
  use: [FILE]
```

**`buf.gen.yaml` (managed mode, Go + TS)**

```yaml
version: v2

managed:
  enabled: true
  go_package_prefix:
    # Use your go.mod module path verbatim:
    # e.g., module github.com/runecraft-studios/pastello
    default: github.com/runecraft-studios/pastello/internal/gen

plugins:
  - plugin: buf.build/protocolbuffers/go
    out: .
    opt: [paths=source_relative]

  # add if you define RPC services:
  # - plugin: buf.build/grpc/go
  #   out: .
  #   opt: [paths=source_relative, require_unimplemented_servers=false]

  - plugin: buf.build/community/stephenh-ts-proto
    out: frontend/src/proto
    opt:
      - env=browser
      - esModuleInterop=true
      - outputServices=none
      - oneof=unions
      - useExactTypes=false
```

# Why we chose this structure

* **Company-first namespace** (`runcraftstudios.pastello.…`): future-proof if you add more apps/sites later, while still scoping to this product (`pastello`).
* **Feature-scoped packages** (`game/trivia/v1`, `game/puzzle/v1`): avoids type/name collisions, lets each game version independently (e.g., `trivia/v2` while `puzzle/v1` stays).
* **Shared types separated** (`game/types/v1`, `game/session/v1`): central spot for cross-game enums and session contracts; keeps feature packages clean.
* **Edge contracts isolated** (`web/game/v1`): envelopes & transport-facing messages live away from domain/game types; makes it easy to evolve the WebSocket/HTTP boundary.
* **Sources vs generated kept apart**:

  * `.proto` live only under `proto/`
  * **Go** output under `internal/gen/...` via `go_package_prefix`
  * **TS** output under `frontend/src/proto/...`
    This prevents mixing IDL with generated code and keeps imports predictable.
* **Managed mode**: Buf auto-derives `go_package` from the package + prefix, so you don’t sprinkle `option go_package` lines everywhere.
* **No dashes in proto packages/folders**: protobuf identifiers forbid `-`. We keep dashes only in the **Go module path** (`github.com/runecraft-studios/...`), which is fine.
* **Avoid duplicate well-known types**: we do **not** add `node_modules` (or any extra `google/`) to include paths; Buf/protoc provide them—prevents the “already defined” and `descriptor.proto` errors.

# Day-to-day workflow

```bash
# after editing proto files
buf lint
buf generate

# backend
go build ./...

# frontend
cd frontend
pnpm dev
```

If you ever need to move/rename packages, do it as a **versioned change** (new `v2` or a new namespace), and Buf’s breaking checks will help you keep consumers safe.
