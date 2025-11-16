/* eslint-disable */
import _m0 from "protobufjs/minimal";
import { Timestamp } from "../../../../../google/protobuf/timestamp";

export const protobufPackage = "runecraftstudios.pastello.web.auth.v1";

export interface StartUserSessionCommand {
  clientVersion: string;
}

export interface SessionStartedEvent {
  sessionId: string;
  userId: string;
  createdAt?: Date | undefined;
}

export interface Envelope {
  correlationId: string;
  msg?: { $case: "startUserSessionCommand"; startUserSessionCommand: StartUserSessionCommand } | {
    $case: "sessionStartedEvent";
    sessionStartedEvent: SessionStartedEvent;
  } | undefined;
}

function createBaseStartUserSessionCommand(): StartUserSessionCommand {
  return { clientVersion: "" };
}

export const StartUserSessionCommand = {
  encode(message: StartUserSessionCommand, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.clientVersion !== "") {
      writer.uint32(10).string(message.clientVersion);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): StartUserSessionCommand {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseStartUserSessionCommand();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.clientVersion = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): StartUserSessionCommand {
    return { clientVersion: isSet(object.clientVersion) ? globalThis.String(object.clientVersion) : "" };
  },

  toJSON(message: StartUserSessionCommand): unknown {
    const obj: any = {};
    if (message.clientVersion !== "") {
      obj.clientVersion = message.clientVersion;
    }
    return obj;
  },

  create(base?: DeepPartial<StartUserSessionCommand>): StartUserSessionCommand {
    return StartUserSessionCommand.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<StartUserSessionCommand>): StartUserSessionCommand {
    const message = createBaseStartUserSessionCommand();
    message.clientVersion = object.clientVersion ?? "";
    return message;
  },
};

function createBaseSessionStartedEvent(): SessionStartedEvent {
  return { sessionId: "", userId: "", createdAt: undefined };
}

export const SessionStartedEvent = {
  encode(message: SessionStartedEvent, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.sessionId !== "") {
      writer.uint32(10).string(message.sessionId);
    }
    if (message.userId !== "") {
      writer.uint32(18).string(message.userId);
    }
    if (message.createdAt !== undefined) {
      Timestamp.encode(toTimestamp(message.createdAt), writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SessionStartedEvent {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSessionStartedEvent();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.sessionId = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.userId = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.createdAt = fromTimestamp(Timestamp.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SessionStartedEvent {
    return {
      sessionId: isSet(object.sessionId) ? globalThis.String(object.sessionId) : "",
      userId: isSet(object.userId) ? globalThis.String(object.userId) : "",
      createdAt: isSet(object.createdAt) ? fromJsonTimestamp(object.createdAt) : undefined,
    };
  },

  toJSON(message: SessionStartedEvent): unknown {
    const obj: any = {};
    if (message.sessionId !== "") {
      obj.sessionId = message.sessionId;
    }
    if (message.userId !== "") {
      obj.userId = message.userId;
    }
    if (message.createdAt !== undefined) {
      obj.createdAt = message.createdAt.toISOString();
    }
    return obj;
  },

  create(base?: DeepPartial<SessionStartedEvent>): SessionStartedEvent {
    return SessionStartedEvent.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<SessionStartedEvent>): SessionStartedEvent {
    const message = createBaseSessionStartedEvent();
    message.sessionId = object.sessionId ?? "";
    message.userId = object.userId ?? "";
    message.createdAt = object.createdAt ?? undefined;
    return message;
  },
};

function createBaseEnvelope(): Envelope {
  return { correlationId: "", msg: undefined };
}

export const Envelope = {
  encode(message: Envelope, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.correlationId !== "") {
      writer.uint32(802).string(message.correlationId);
    }
    switch (message.msg?.$case) {
      case "startUserSessionCommand":
        StartUserSessionCommand.encode(message.msg.startUserSessionCommand, writer.uint32(10).fork()).ldelim();
        break;
      case "sessionStartedEvent":
        SessionStartedEvent.encode(message.msg.sessionStartedEvent, writer.uint32(18).fork()).ldelim();
        break;
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Envelope {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEnvelope();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 100:
          if (tag !== 802) {
            break;
          }

          message.correlationId = reader.string();
          continue;
        case 1:
          if (tag !== 10) {
            break;
          }

          message.msg = {
            $case: "startUserSessionCommand",
            startUserSessionCommand: StartUserSessionCommand.decode(reader, reader.uint32()),
          };
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.msg = {
            $case: "sessionStartedEvent",
            sessionStartedEvent: SessionStartedEvent.decode(reader, reader.uint32()),
          };
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Envelope {
    return {
      correlationId: isSet(object.correlationId) ? globalThis.String(object.correlationId) : "",
      msg: isSet(object.startUserSessionCommand)
        ? {
          $case: "startUserSessionCommand",
          startUserSessionCommand: StartUserSessionCommand.fromJSON(object.startUserSessionCommand),
        }
        : isSet(object.sessionStartedEvent)
        ? {
          $case: "sessionStartedEvent",
          sessionStartedEvent: SessionStartedEvent.fromJSON(object.sessionStartedEvent),
        }
        : undefined,
    };
  },

  toJSON(message: Envelope): unknown {
    const obj: any = {};
    if (message.correlationId !== "") {
      obj.correlationId = message.correlationId;
    }
    if (message.msg?.$case === "startUserSessionCommand") {
      obj.startUserSessionCommand = StartUserSessionCommand.toJSON(message.msg.startUserSessionCommand);
    }
    if (message.msg?.$case === "sessionStartedEvent") {
      obj.sessionStartedEvent = SessionStartedEvent.toJSON(message.msg.sessionStartedEvent);
    }
    return obj;
  },

  create(base?: DeepPartial<Envelope>): Envelope {
    return Envelope.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<Envelope>): Envelope {
    const message = createBaseEnvelope();
    message.correlationId = object.correlationId ?? "";
    if (
      object.msg?.$case === "startUserSessionCommand" &&
      object.msg?.startUserSessionCommand !== undefined &&
      object.msg?.startUserSessionCommand !== null
    ) {
      message.msg = {
        $case: "startUserSessionCommand",
        startUserSessionCommand: StartUserSessionCommand.fromPartial(object.msg.startUserSessionCommand),
      };
    }
    if (
      object.msg?.$case === "sessionStartedEvent" &&
      object.msg?.sessionStartedEvent !== undefined &&
      object.msg?.sessionStartedEvent !== null
    ) {
      message.msg = {
        $case: "sessionStartedEvent",
        sessionStartedEvent: SessionStartedEvent.fromPartial(object.msg.sessionStartedEvent),
      };
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
