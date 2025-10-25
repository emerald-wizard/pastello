# Project Architecture Overview

## Introduction

This document describes the project's architecture, detailing the organization of files, modules, and their interactions. It also explicitly connects architectural components with the identified workflow steps.

## Folder Structure

```
goproject_with_files/
├── cmd/
│   ├── app/                    # Initializes HTTP/Websocket server
│   └── worker/                 # Event processing (optional)
├── internal/
│   ├── domain/                 # Core business entities and logic
│   │   ├── game/               # Game logic (Questions, Participants, Events)
│   │   └── security/           # Authentication & authorization logic
│   ├── application/            # Use cases, commands, queries, handlers
│   │   ├── commands/
│   │   ├── queries/
│   │   ├── services/
│   │   ├── command_bus.go      # Command routing
│   │   └── ports/
│   │       ├── inbound/
│   │       │   ├── rest/       # REST endpoints (admin/game setup)
│   │       │   └── websocket/  # WebSocket communication handlers
│   │       └── outbound/
│   │           ├── broadcaster.go
│   │           └── game_repository.go
│   ├── adapters/               # Infrastructure implementation
│   │   ├── persistence/
│   │   ├── externalapi/
│   │   └── messaging/
│   └── utils/                  # Shared utilities
├── migrations/                 # Database migrations
├── test/                       # Integration & E2E tests
└── pkg/                        # Reusable infrastructure components
```

## Detailed Component Description

### `cmd/app`

* Initializes and starts the HTTP and WebSocket servers.
* Entry point linked to \[Step A1] Page Load, triggering the authentication and role determination flow.

### `internal/domain/security/auth.go`

* Handles authentication logic, validates tokens, and retrieves user profiles.
* Implements steps \[A2.1.2], \[A2.1.2.1], and \[A2.1.2.2.x].

### `internal/application/ports/inbound/rest`

* Provides REST APIs used for validating tokens and fetching user profile data (\[A2.1.2]).

### `internal/application/ports/inbound/websocket`

* `handler.go`: Manages WebSocket connections (Step \[C1.1]).
* `broadcaster.go`: Sends real-time game updates to clients after successful session establishment (\[C1.1]).

### `internal/application/services/game_manager.go`

* Coordinates game lifecycle and state, initiates session lookups and session management (\[B2] session lookup steps).
* Coordinates between the domain and persistence adapters to manage game state.

### `internal/adapters/persistence/game_repository.go`

* Implements `game_repository` port, handles the persistence of game state.
* Supports steps \[B2.x] for game session validation and management.

## Workflow and Architecture Mapping

| Step ID          | Component(s)                                                          | Description                              |
| ---------------- | --------------------------------------------------------------------- | ---------------------------------------- |
| A1               | `cmd/app`                                                             | Initial page load and trigger flow.      |
| A2, A2.1         | Frontend/browser, indirectly via REST API                             | Role determination initiated.            |
| A2.1.2, A2.1.2.1 | `internal/domain/security/auth.go`, `ports/inbound/rest`              | Token validation/authentication.         |
| A2.1.2.2.x       | `internal/domain/security/auth.go`                                    | User profile retrieval (admin/member)    |
| B1               | Frontend                                                              | Login/signup UI for visitors.            |
| B2.x             | `internal/application/services/game_manager.go`, persistence adapters | Session lookup and management.           |
| C1, C1.1         | `internal/application/ports/inbound/websocket`, `adapters/messaging`  | WebSocket initialization and management. |

## Architectural Patterns and Principles

* **CQRS:** Separates read (queries) and write (commands) operations, clearly reflected in application structure.
* **Hexagonal (Ports & Adapters):** Ensures domain purity and flexibility by abstracting infrastructure concerns.
* **Clean Architecture:** Dependency flows inward from adapters through application services to domain.

## Testing Strategy

* Integration tests (`test/integration`) cover interactions between adapters, ports, and external services.
* End-to-end tests (`test/e2e`) validate system behavior from HTTP/WebSocket endpoints through domain logic.

## Maintaining this Document

* Update when changes occur in folder structure or logical partitions.
* Maintain mapping between step IDs and architectural components to ensure clarity and ease of navigation.

This document serves as a foundational reference to navigate, understand, and maintain the system architecture, explicitly linking it to the defined workflow and traceability system.
