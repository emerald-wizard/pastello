/* eslint-disable */
import _m0 from "protobufjs/minimal";
import { Timestamp } from "../../../../../google/protobuf/timestamp";

export const protobufPackage = "runecraftstudios.pastello.game.types.v1";

/** Enums */
export enum GameType {
  GAME_TYPE_UNSPECIFIED = 0,
  GAME_TYPE_TRIVIA = 1,
  GAME_TYPE_PUZZLE = 2,
  UNRECOGNIZED = -1,
}

export function gameTypeFromJSON(object: any): GameType {
  switch (object) {
    case 0:
    case "GAME_TYPE_UNSPECIFIED":
      return GameType.GAME_TYPE_UNSPECIFIED;
    case 1:
    case "GAME_TYPE_TRIVIA":
      return GameType.GAME_TYPE_TRIVIA;
    case 2:
    case "GAME_TYPE_PUZZLE":
      return GameType.GAME_TYPE_PUZZLE;
    case -1:
    case "UNRECOGNIZED":
    default:
      return GameType.UNRECOGNIZED;
  }
}

export function gameTypeToJSON(object: GameType): string {
  switch (object) {
    case GameType.GAME_TYPE_UNSPECIFIED:
      return "GAME_TYPE_UNSPECIFIED";
    case GameType.GAME_TYPE_TRIVIA:
      return "GAME_TYPE_TRIVIA";
    case GameType.GAME_TYPE_PUZZLE:
      return "GAME_TYPE_PUZZLE";
    case GameType.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export enum SessionStatus {
  SESSION_STATUS_UNSPECIFIED = 0,
  SESSION_STATUS_CREATED = 1,
  SESSION_STATUS_ACTIVE = 2,
  SESSION_STATUS_ENDED = 3,
  SESSION_STATUS_CANCELLED = 4,
  UNRECOGNIZED = -1,
}

export function sessionStatusFromJSON(object: any): SessionStatus {
  switch (object) {
    case 0:
    case "SESSION_STATUS_UNSPECIFIED":
      return SessionStatus.SESSION_STATUS_UNSPECIFIED;
    case 1:
    case "SESSION_STATUS_CREATED":
      return SessionStatus.SESSION_STATUS_CREATED;
    case 2:
    case "SESSION_STATUS_ACTIVE":
      return SessionStatus.SESSION_STATUS_ACTIVE;
    case 3:
    case "SESSION_STATUS_ENDED":
      return SessionStatus.SESSION_STATUS_ENDED;
    case 4:
    case "SESSION_STATUS_CANCELLED":
      return SessionStatus.SESSION_STATUS_CANCELLED;
    case -1:
    case "UNRECOGNIZED":
    default:
      return SessionStatus.UNRECOGNIZED;
  }
}

export function sessionStatusToJSON(object: SessionStatus): string {
  switch (object) {
    case SessionStatus.SESSION_STATUS_UNSPECIFIED:
      return "SESSION_STATUS_UNSPECIFIED";
    case SessionStatus.SESSION_STATUS_CREATED:
      return "SESSION_STATUS_CREATED";
    case SessionStatus.SESSION_STATUS_ACTIVE:
      return "SESSION_STATUS_ACTIVE";
    case SessionStatus.SESSION_STATUS_ENDED:
      return "SESSION_STATUS_ENDED";
    case SessionStatus.SESSION_STATUS_CANCELLED:
      return "SESSION_STATUS_CANCELLED";
    case SessionStatus.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

/** IDs */
export interface GameSessionId {
  value: string;
}

export interface PlayerId {
  value: string;
}

/** Optional audit helper (use when you need it) */
export interface Audit {
  /**  */
  dummySlot: number;
  createdAt?: Date | undefined;
  updatedAt?: Date | undefined;
}

function createBaseGameSessionId(): GameSessionId {
  return { value: "" };
}

export const GameSessionId = {
  encode(message: GameSessionId, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.value !== "") {
      writer.uint32(10).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GameSessionId {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGameSessionId();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.value = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): GameSessionId {
    return { value: isSet(object.value) ? globalThis.String(object.value) : "" };
  },

  toJSON(message: GameSessionId): unknown {
    const obj: any = {};
    if (message.value !== "") {
      obj.value = message.value;
    }
    return obj;
  },

  create(base?: DeepPartial<GameSessionId>): GameSessionId {
    return GameSessionId.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<GameSessionId>): GameSessionId {
    const message = createBaseGameSessionId();
    message.value = object.value ?? "";
    return message;
  },
};

function createBasePlayerId(): PlayerId {
  return { value: "" };
}

export const PlayerId = {
  encode(message: PlayerId, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.value !== "") {
      writer.uint32(10).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PlayerId {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePlayerId();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.value = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): PlayerId {
    return { value: isSet(object.value) ? globalThis.String(object.value) : "" };
  },

  toJSON(message: PlayerId): unknown {
    const obj: any = {};
    if (message.value !== "") {
      obj.value = message.value;
    }
    return obj;
  },

  create(base?: DeepPartial<PlayerId>): PlayerId {
    return PlayerId.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<PlayerId>): PlayerId {
    const message = createBasePlayerId();
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseAudit(): Audit {
  return { dummySlot: 0, createdAt: undefined, updatedAt: undefined };
}

export const Audit = {
  encode(message: Audit, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.dummySlot !== 0) {
      writer.uint32(8).int32(message.dummySlot);
    }
    if (message.createdAt !== undefined) {
      Timestamp.encode(toTimestamp(message.createdAt), writer.uint32(18).fork()).ldelim();
    }
    if (message.updatedAt !== undefined) {
      Timestamp.encode(toTimestamp(message.updatedAt), writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Audit {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAudit();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.dummySlot = reader.int32();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.createdAt = fromTimestamp(Timestamp.decode(reader, reader.uint32()));
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.updatedAt = fromTimestamp(Timestamp.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Audit {
    return {
      dummySlot: isSet(object.dummySlot) ? globalThis.Number(object.dummySlot) : 0,
      createdAt: isSet(object.createdAt) ? fromJsonTimestamp(object.createdAt) : undefined,
      updatedAt: isSet(object.updatedAt) ? fromJsonTimestamp(object.updatedAt) : undefined,
    };
  },

  toJSON(message: Audit): unknown {
    const obj: any = {};
    if (message.dummySlot !== 0) {
      obj.dummySlot = Math.round(message.dummySlot);
    }
    if (message.createdAt !== undefined) {
      obj.createdAt = message.createdAt.toISOString();
    }
    if (message.updatedAt !== undefined) {
      obj.updatedAt = message.updatedAt.toISOString();
    }
    return obj;
  },

  create(base?: DeepPartial<Audit>): Audit {
    return Audit.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<Audit>): Audit {
    const message = createBaseAudit();
    message.dummySlot = object.dummySlot ?? 0;
    message.createdAt = object.createdAt ?? undefined;
    message.updatedAt = object.updatedAt ?? undefined;
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

function toTimestamp(date: Date): Timestamp {
  const seconds = Math.trunc(date.getTime() / 1_000);
  const nanos = (date.getTime() % 1_000) * 1_000_000;
  return { seconds, nanos };
}

function fromTimestamp(t: Timestamp): Date {
  let millis = (t.seconds || 0) * 1_000;
  millis += (t.nanos || 0) / 1_000_000;
  return new globalThis.Date(millis);
}

function fromJsonTimestamp(o: any): Date {
  if (o instanceof globalThis.Date) {
    return o;
  } else if (typeof o === "string") {
    return new globalThis.Date(o);
  } else {
    return fromTimestamp(Timestamp.fromJSON(o));
  }
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
