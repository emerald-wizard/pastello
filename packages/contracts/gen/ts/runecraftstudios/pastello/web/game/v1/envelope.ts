/* eslint-disable */
import _m0 from "protobufjs/minimal";
import { MovePieceCommand, UndoMoveCommand } from "../../../game/puzzle/v1/commands";
import { RevealHintCommand, SubmitAnswerCommand } from "../../../game/trivia/v1/commands";
import { GameType, gameTypeFromJSON, gameTypeToJSON } from "../../../game/types/v1/types";

export const protobufPackage = "runecraftstudios.pastello.web.game.v1";

/**
 * ClientEnvelope is the top-level message from the client over WebSocket.
 * The Rust code `ws/mod.rs` expects this structure.
 */
export interface ClientEnvelope {
  message?: { $case: "startGame"; startGame: StartGameCommand } | {
    $case: "gameCommand";
    gameCommand: GameCommandEnvelope;
  } | undefined;
}

/**
 * Sent by the client to initiate a game session.
 * The Rust code `ws/mod.rs` expects this to have a `game_type` field.
 */
export interface StartGameCommand {
  gameType: GameType;
}

/**
 * GameCommandEnvelope wraps all in-game actions.
 * The Rust code `ws/mod.rs` expects this structure.
 */
export interface GameCommandEnvelope {
  command?:
    | { $case: "puzzleMove"; puzzleMove: MovePieceCommand }
    | { $case: "puzzleUndo"; puzzleUndo: UndoMoveCommand }
    | { $case: "triviaSubmit"; triviaSubmit: SubmitAnswerCommand }
    | { $case: "triviaHint"; triviaHint: RevealHintCommand }
    | undefined;
}

/**
 * ServerEnvelope is the top-level message from the server to the client.
 * Your Rust code will need this to send events and status updates back.
 */
export interface ServerEnvelope {
  message?:
    | { $case: "authStatus"; authStatus: string }
    | { $case: "gameState"; gameState: string }
    | { $case: "error"; error: string }
    | { $case: "gameEvent"; gameEvent: GameEventEnvelope }
    | undefined;
}

/** GameEventEnvelope wraps all game-specific events. */
export interface GameEventEnvelope {
  event?: { $case: "puzzleEvent"; puzzleEvent: string } | { $case: "triviaEvent"; triviaEvent: string } | undefined;
}

function createBaseClientEnvelope(): ClientEnvelope {
  return { message: undefined };
}

export const ClientEnvelope = {
  encode(message: ClientEnvelope, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    switch (message.message?.$case) {
      case "startGame":
        StartGameCommand.encode(message.message.startGame, writer.uint32(10).fork()).ldelim();
        break;
      case "gameCommand":
        GameCommandEnvelope.encode(message.message.gameCommand, writer.uint32(18).fork()).ldelim();
        break;
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ClientEnvelope {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseClientEnvelope();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.message = { $case: "startGame", startGame: StartGameCommand.decode(reader, reader.uint32()) };
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.message = { $case: "gameCommand", gameCommand: GameCommandEnvelope.decode(reader, reader.uint32()) };
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ClientEnvelope {
    return {
      message: isSet(object.startGame)
        ? { $case: "startGame", startGame: StartGameCommand.fromJSON(object.startGame) }
        : isSet(object.gameCommand)
        ? { $case: "gameCommand", gameCommand: GameCommandEnvelope.fromJSON(object.gameCommand) }
        : undefined,
    };
  },

  toJSON(message: ClientEnvelope): unknown {
    const obj: any = {};
    if (message.message?.$case === "startGame") {
      obj.startGame = StartGameCommand.toJSON(message.message.startGame);
    }
    if (message.message?.$case === "gameCommand") {
      obj.gameCommand = GameCommandEnvelope.toJSON(message.message.gameCommand);
    }
    return obj;
  },

  create(base?: DeepPartial<ClientEnvelope>): ClientEnvelope {
    return ClientEnvelope.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<ClientEnvelope>): ClientEnvelope {
    const message = createBaseClientEnvelope();
    if (
      object.message?.$case === "startGame" &&
      object.message?.startGame !== undefined &&
      object.message?.startGame !== null
    ) {
      message.message = { $case: "startGame", startGame: StartGameCommand.fromPartial(object.message.startGame) };
    }
    if (
      object.message?.$case === "gameCommand" &&
      object.message?.gameCommand !== undefined &&
      object.message?.gameCommand !== null
    ) {
      message.message = {
        $case: "gameCommand",
        gameCommand: GameCommandEnvelope.fromPartial(object.message.gameCommand),
      };
    }
    return message;
  },
};

function createBaseStartGameCommand(): StartGameCommand {
  return { gameType: 0 };
}

export const StartGameCommand = {
  encode(message: StartGameCommand, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.gameType !== 0) {
      writer.uint32(8).int32(message.gameType);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): StartGameCommand {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseStartGameCommand();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.gameType = reader.int32() as any;
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): StartGameCommand {
    return { gameType: isSet(object.gameType) ? gameTypeFromJSON(object.gameType) : 0 };
  },

  toJSON(message: StartGameCommand): unknown {
    const obj: any = {};
    if (message.gameType !== 0) {
      obj.gameType = gameTypeToJSON(message.gameType);
    }
    return obj;
  },

  create(base?: DeepPartial<StartGameCommand>): StartGameCommand {
    return StartGameCommand.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<StartGameCommand>): StartGameCommand {
    const message = createBaseStartGameCommand();
    message.gameType = object.gameType ?? 0;
    return message;
  },
};

function createBaseGameCommandEnvelope(): GameCommandEnvelope {
  return { command: undefined };
}

export const GameCommandEnvelope = {
  encode(message: GameCommandEnvelope, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    switch (message.command?.$case) {
      case "puzzleMove":
        MovePieceCommand.encode(message.command.puzzleMove, writer.uint32(10).fork()).ldelim();
        break;
      case "puzzleUndo":
        UndoMoveCommand.encode(message.command.puzzleUndo, writer.uint32(18).fork()).ldelim();
        break;
      case "triviaSubmit":
        SubmitAnswerCommand.encode(message.command.triviaSubmit, writer.uint32(26).fork()).ldelim();
        break;
      case "triviaHint":
        RevealHintCommand.encode(message.command.triviaHint, writer.uint32(34).fork()).ldelim();
        break;
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameCommandEnvelope {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGameCommandEnvelope();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.command = { $case: "puzzleMove", puzzleMove: MovePieceCommand.decode(reader, reader.uint32()) };
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.command = { $case: "puzzleUndo", puzzleUndo: UndoMoveCommand.decode(reader, reader.uint32()) };
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.command = {
            $case: "triviaSubmit",
            triviaSubmit: SubmitAnswerCommand.decode(reader, reader.uint32()),
          };
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.command = { $case: "triviaHint", triviaHint: RevealHintCommand.decode(reader, reader.uint32()) };
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): GameCommandEnvelope {
    return {
      command: isSet(object.puzzleMove)
        ? { $case: "puzzleMove", puzzleMove: MovePieceCommand.fromJSON(object.puzzleMove) }
        : isSet(object.puzzleUndo)
        ? { $case: "puzzleUndo", puzzleUndo: UndoMoveCommand.fromJSON(object.puzzleUndo) }
        : isSet(object.triviaSubmit)
        ? { $case: "triviaSubmit", triviaSubmit: SubmitAnswerCommand.fromJSON(object.triviaSubmit) }
        : isSet(object.triviaHint)
        ? { $case: "triviaHint", triviaHint: RevealHintCommand.fromJSON(object.triviaHint) }
        : undefined,
    };
  },

  toJSON(message: GameCommandEnvelope): unknown {
    const obj: any = {};
    if (message.command?.$case === "puzzleMove") {
      obj.puzzleMove = MovePieceCommand.toJSON(message.command.puzzleMove);
    }
    if (message.command?.$case === "puzzleUndo") {
      obj.puzzleUndo = UndoMoveCommand.toJSON(message.command.puzzleUndo);
    }
    if (message.command?.$case === "triviaSubmit") {
      obj.triviaSubmit = SubmitAnswerCommand.toJSON(message.command.triviaSubmit);
    }
    if (message.command?.$case === "triviaHint") {
      obj.triviaHint = RevealHintCommand.toJSON(message.command.triviaHint);
    }
    return obj;
  },

  create(base?: DeepPartial<GameCommandEnvelope>): GameCommandEnvelope {
    return GameCommandEnvelope.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<GameCommandEnvelope>): GameCommandEnvelope {
    const message = createBaseGameCommandEnvelope();
    if (
      object.command?.$case === "puzzleMove" &&
      object.command?.puzzleMove !== undefined &&
      object.command?.puzzleMove !== null
    ) {
      message.command = { $case: "puzzleMove", puzzleMove: MovePieceCommand.fromPartial(object.command.puzzleMove) };
    }
    if (
      object.command?.$case === "puzzleUndo" &&
      object.command?.puzzleUndo !== undefined &&
      object.command?.puzzleUndo !== null
    ) {
      message.command = { $case: "puzzleUndo", puzzleUndo: UndoMoveCommand.fromPartial(object.command.puzzleUndo) };
    }
    if (
      object.command?.$case === "triviaSubmit" &&
      object.command?.triviaSubmit !== undefined &&
      object.command?.triviaSubmit !== null
    ) {
      message.command = {
        $case: "triviaSubmit",
        triviaSubmit: SubmitAnswerCommand.fromPartial(object.command.triviaSubmit),
      };
    }
    if (
      object.command?.$case === "triviaHint" &&
      object.command?.triviaHint !== undefined &&
      object.command?.triviaHint !== null
    ) {
      message.command = { $case: "triviaHint", triviaHint: RevealHintCommand.fromPartial(object.command.triviaHint) };
    }
    return message;
  },
};

function createBaseServerEnvelope(): ServerEnvelope {
  return { message: undefined };
}

export const ServerEnvelope = {
  encode(message: ServerEnvelope, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    switch (message.message?.$case) {
      case "authStatus":
        writer.uint32(10).string(message.message.authStatus);
        break;
      case "gameState":
        writer.uint32(18).string(message.message.gameState);
        break;
      case "error":
        writer.uint32(26).string(message.message.error);
        break;
      case "gameEvent":
        GameEventEnvelope.encode(message.message.gameEvent, writer.uint32(34).fork()).ldelim();
        break;
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ServerEnvelope {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseServerEnvelope();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.message = { $case: "authStatus", authStatus: reader.string() };
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.message = { $case: "gameState", gameState: reader.string() };
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.message = { $case: "error", error: reader.string() };
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.message = { $case: "gameEvent", gameEvent: GameEventEnvelope.decode(reader, reader.uint32()) };
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ServerEnvelope {
    return {
      message: isSet(object.authStatus)
        ? { $case: "authStatus", authStatus: globalThis.String(object.authStatus) }
        : isSet(object.gameState)
        ? { $case: "gameState", gameState: globalThis.String(object.gameState) }
        : isSet(object.error)
        ? { $case: "error", error: globalThis.String(object.error) }
        : isSet(object.gameEvent)
        ? { $case: "gameEvent", gameEvent: GameEventEnvelope.fromJSON(object.gameEvent) }
        : undefined,
    };
  },

  toJSON(message: ServerEnvelope): unknown {
    const obj: any = {};
    if (message.message?.$case === "authStatus") {
      obj.authStatus = message.message.authStatus;
    }
    if (message.message?.$case === "gameState") {
      obj.gameState = message.message.gameState;
    }
    if (message.message?.$case === "error") {
      obj.error = message.message.error;
    }
    if (message.message?.$case === "gameEvent") {
      obj.gameEvent = GameEventEnvelope.toJSON(message.message.gameEvent);
    }
    return obj;
  },

  create(base?: DeepPartial<ServerEnvelope>): ServerEnvelope {
    return ServerEnvelope.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<ServerEnvelope>): ServerEnvelope {
    const message = createBaseServerEnvelope();
    if (
      object.message?.$case === "authStatus" &&
      object.message?.authStatus !== undefined &&
      object.message?.authStatus !== null
    ) {
      message.message = { $case: "authStatus", authStatus: object.message.authStatus };
    }
    if (
      object.message?.$case === "gameState" &&
      object.message?.gameState !== undefined &&
      object.message?.gameState !== null
    ) {
      message.message = { $case: "gameState", gameState: object.message.gameState };
    }
    if (object.message?.$case === "error" && object.message?.error !== undefined && object.message?.error !== null) {
      message.message = { $case: "error", error: object.message.error };
    }
    if (
      object.message?.$case === "gameEvent" &&
      object.message?.gameEvent !== undefined &&
      object.message?.gameEvent !== null
    ) {
      message.message = { $case: "gameEvent", gameEvent: GameEventEnvelope.fromPartial(object.message.gameEvent) };
    }
    return message;
  },
};

function createBaseGameEventEnvelope(): GameEventEnvelope {
  return { event: undefined };
}

export const GameEventEnvelope = {
  encode(message: GameEventEnvelope, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    switch (message.event?.$case) {
      case "puzzleEvent":
        writer.uint32(10).string(message.event.puzzleEvent);
        break;
      case "triviaEvent":
        writer.uint32(18).string(message.event.triviaEvent);
        break;
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameEventEnvelope {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGameEventEnvelope();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.event = { $case: "puzzleEvent", puzzleEvent: reader.string() };
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.event = { $case: "triviaEvent", triviaEvent: reader.string() };
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): GameEventEnvelope {
    return {
      event: isSet(object.puzzleEvent)
        ? { $case: "puzzleEvent", puzzleEvent: globalThis.String(object.puzzleEvent) }
        : isSet(object.triviaEvent)
        ? { $case: "triviaEvent", triviaEvent: globalThis.String(object.triviaEvent) }
        : undefined,
    };
  },

  toJSON(message: GameEventEnvelope): unknown {
    const obj: any = {};
    if (message.event?.$case === "puzzleEvent") {
      obj.puzzleEvent = message.event.puzzleEvent;
    }
    if (message.event?.$case === "triviaEvent") {
      obj.triviaEvent = message.event.triviaEvent;
    }
    return obj;
  },

  create(base?: DeepPartial<GameEventEnvelope>): GameEventEnvelope {
    return GameEventEnvelope.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<GameEventEnvelope>): GameEventEnvelope {
    const message = createBaseGameEventEnvelope();
    if (
      object.event?.$case === "puzzleEvent" &&
      object.event?.puzzleEvent !== undefined &&
      object.event?.puzzleEvent !== null
    ) {
      message.event = { $case: "puzzleEvent", puzzleEvent: object.event.puzzleEvent };
    }
    if (
      object.event?.$case === "triviaEvent" &&
      object.event?.triviaEvent !== undefined &&
      object.event?.triviaEvent !== null
    ) {
      message.event = { $case: "triviaEvent", triviaEvent: object.event.triviaEvent };
    }
    return message;
  },
};

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends { $case: string } ? { [K in keyof Omit<T, "$case">]?: DeepPartial<T[K]> } & { $case: T["$case"] }
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
