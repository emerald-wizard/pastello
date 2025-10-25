Here’s a compact “project brief” you can paste into a new chat so we start with full context.

# Project snapshot

## Goals

* Build a **Vue 3 + TypeScript** SPA that talks to a **Go** backend over **WebSockets** using **Protocol Buffers** as the shared contract.
* Follow **Clean/Hexagonal/DDD** with a pragmatic twist: **Use Cases** internally + a thin **Service façade** at the edge.
* Generate client/server protobuf code with **Buf v2**.

---

## Frontend

* **Stack**: Vue 3 (SFC + Composition API) + TypeScript, Vite, pnpm.
* **Wire protocol**: WebSocket **binary** frames carrying protobuf messages.
* **Proto → TS**: Generated with **ts-proto** via Buf.

  * Runtime deps: `"protobufjs"` and `"long"`.
  * Optional init:

    ```ts
    import _m0 from "protobufjs/minimal";
    import Long from "long";
    if (_m0.util.Long !== Long) { _m0.util.Long = Long as any; _m0.configure(); }
    ```
* **Message design**: top-level **envelope** for correlation + oneof payloads (e.g., `StartGameSessionCommand`, game commands, events).
* **State**: client updates UI from server events (e.g., `GameSessionStartedEvent`, `TriviaAnswerAccepted`).

---

## Backend

* **Language**: Go.
* **Transport**: gorilla/websocket (binary messages).
* **Architecture**:

  * **Domain**: game engines (trivia/puzzle), aggregates (`GameSession`), rules.
  * **Application**:

    * **Use cases** (one action per file) with `Execute(ctx, In) (Out, error)`.
    * **Services façade** → thin pass-through to UCs (prevents god services, keeps handlers friendly).
    * (Optional) **Queries** folder if/when adding read-side CQRS.
  * **Ports (Outbound)**: `GameSessionRepo`, `EventBus`, `Clock`, `IDGen` (interfaces only).
  * **Adapters**:

    * **Inbound** (`internal/adapters/inbound/ws`):

      * `ws_server.go` (upgrade + read loop)
      * `router.go` (inspects binary payloads and routes)
      * `game_start_handler.go`, `game_command_handler.go` (decode → service → encode)
      * Router uses tiny interfaces (`StartEnvelopeHandler`, `CommandFrameHandler`); concretes `GameStartHandler`, `GameCommandHandler` already satisfy them.
    * **Outbound**:

      * `memory` (in-memory repo), `eventbus` (NOP/optional logger), `rng` (crypto random IDs).
* **Command Registry**: maps incoming command envelopes → appropriate façade methods → emits events.
* **Binary**: `cmd/server/main.go` wires adapters and services; listen on `:8080`.

---

## Protobuf & Buf (v2)

* **Namespace**: company-first, then product:

  * `runecraftstudios.pastello.…`
* **Packages & folders** (feature-scoped):

  ```
  proto/
    runecraftstudios/
      pastello/
        game/
          types/v1/        # shared enums/IDs
          session/v1/      # shared session API (Create/Start/etc.)
          trivia/v1/       # trivia-specific rules/commands/events
          puzzle/v1/       # puzzle-specific rules/commands/events
        web/
          game/v1/         # edge envelopes for WS/HTTP
  ```

  * **Rule**: package line must 1:1 match folders; no dashes in proto package segments.
  * Always **import** cross-package types (compiler doesn’t infer).
* **Lint**: `STANDARD` (we replaced deprecated `DEFAULT`).

  * Enum style: either follow `ENUM_VALUE_PREFIX` (portable) **or** disable it via:

    ```yaml
    lint:
      use: [STANDARD]
      except: [ENUM_VALUE_PREFIX]
    ```
  * Keep `UNSPECIFIED = 0` enum value.
* **Generation outputs**:

  * **Go** → `internal/gen/runecraftstudios/pastello/...`
  * **TS** → `frontend/src/proto/runecraftstudios/pastello/...`
* **Buf v2 configs** (root):

  ```yaml
  # buf.yaml
  version: v2
  modules:
    - path: proto
  lint:
    use: [STANDARD]
  breaking:
    use: [FILE]
  ```

  ```yaml
  # buf.gen.yaml
  version: v2
  managed:
    enabled: true
    override:
      - file_option: go_package_prefix
        value: github.com/runecraft-studios/pastello/internal/gen

  plugins:
    - remote: buf.build/protocolbuffers/go
      out: internal/gen
    # - remote: buf.build/grpc/go
    #   out: internal/gen
    #   opt: [require_unimplemented_servers=false]  # only if you add services
    - remote: buf.build/community/stephenh-ts-proto
      out: frontend/src/proto
      opt:
        - env=browser
        - esModuleInterop=true
        - outputServices=none
        - oneof=unions
        - useExactTypes=false
  ```
* **Commands**:

  ```
  buf --version                      # v2
  buf config ls-modules              # shows: path: proto
  buf ls-files                       # lists included .proto
  buf lint
  buf generate --template buf.gen.yaml --verbose
  ```

---

## Naming & modules

* **Go module**: may include GitHub dash (e.g., `github.com/runecraft-studios/pastello`).
* **Proto packages/folders**: **no dashes** → `runecraftstudios` not `runecraft-studios`.
* **Generated Go import path** is `go_package_prefix` + proto package path.

---

## Current gameplay features

* **Trivia** and **Puzzle (sliding/scramble)** as separate feature packages.
* Shared **session** and **types**; engines encapsulate per-game logic.
* Design supports scaling games into a separate service later (message-driven, versioned packages).

---

## Open/next items

* Map real **rules** proto ⇆ domain in `trivialRulesFromProto`/`puzzleRulesFromProto`.
* Decide enum prefix policy (keep or disable rule).
* Add **authn/authz** at handler or façade (transport concerns at handler; app-wide policy in UCs).
* Optional **queries/** if you expose read models separately.
* (Later) DB-backed repo + UnitOfWork port if persistence is added.

---

## Why this setup

* **Clean boundaries** (proto ↔ adapters ↔ application ↔ domain).
* **Testability** (use-cases are small, transport-free; adapters are thin).
* **Scalability** (feature packages version independently; can split services).
* **DX** (Buf v2 managed codegen; no local protoc plugins to install).

---

If you paste this into a new chat and say “continue from this project brief,” we’ll pick up exactly where we left off.
