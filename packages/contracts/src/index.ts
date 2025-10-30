// packages/contracts/src/index.ts

// Web-facing envelope & auth
export * as web_game_v1 from "../gen/ts/runecraftstudios/pastello/web/game/v1/envelope";
export * as web_auth_v1 from "../gen/ts/runecraftstudios/pastello/web/auth/v1/session";

// Game sessions & shared types
export * as game_session_v1 from "../gen/ts/runecraftstudios/pastello/game/session/v1/session";
export * as game_types_v1 from "../gen/ts/runecraftstudios/pastello/game/types/v1/types";

// Trivia & Puzzle commands/rules (optional)
export * as trivia_cmd_v1 from "../gen/ts/runecraftstudios/pastello/game/trivia/v1/commands";
export * as trivia_rules_v1 from "../gen/ts/runecraftstudios/pastello/game/trivia/v1/rules";
export * as puzzle_cmd_v1 from "../gen/ts/runecraftstudios/pastello/game/puzzle/v1/commands";
export * as puzzle_rules_v1 from "../gen/ts/runecraftstudios/pastello/game/puzzle/v1/rules";

