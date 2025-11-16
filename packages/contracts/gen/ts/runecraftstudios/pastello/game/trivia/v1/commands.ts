/* eslint-disable */
import _m0 from "protobufjs/minimal";
import { GameSessionId, PlayerId } from "../../types/v1/types";

export const protobufPackage = "runecraftstudios.pastello.game.trivia.v1";

export interface SubmitAnswerCommand {
  sessionId?: GameSessionId | undefined;
  playerId?: PlayerId | undefined;
  answer: string;
}

export interface RevealHintCommand {
  sessionId?: GameSessionId | undefined;
}

export interface AnswerAcceptedEvent {
  sessionId?: GameSessionId | undefined;
  playerId?: PlayerId | undefined;
  deltaScore: number;
  totalScore: number;
}

export interface HintRevealedEvent {
  sessionId?: GameSessionId | undefined;
  hintText: string;
}

function createBaseSubmitAnswerCommand(): SubmitAnswerCommand {
  return { sessionId: undefined, playerId: undefined, answer: "" };
}

export const SubmitAnswerCommand = {
  encode(message: SubmitAnswerCommand, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.sessionId !== undefined) {
      GameSessionId.encode(message.sessionId, writer.uint32(10).fork()).ldelim();
    }
    if (message.playerId !== undefined) {
      PlayerId.encode(message.playerId, writer.uint32(18).fork()).ldelim();
    }
    if (message.answer !== "") {
      writer.uint32(26).string(message.answer);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SubmitAnswerCommand {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSubmitAnswerCommand();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.sessionId = GameSessionId.decode(reader, reader.uint32());
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.playerId = PlayerId.decode(reader, reader.uint32());
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.answer = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SubmitAnswerCommand {
    return {
      sessionId: isSet(object.sessionId) ? GameSessionId.fromJSON(object.sessionId) : undefined,
      playerId: isSet(object.playerId) ? PlayerId.fromJSON(object.playerId) : undefined,
      answer: isSet(object.answer) ? globalThis.String(object.answer) : "",
    };
  },

  toJSON(message: SubmitAnswerCommand): unknown {
    const obj: any = {};
    if (message.sessionId !== undefined) {
      obj.sessionId = GameSessionId.toJSON(message.sessionId);
    }
    if (message.playerId !== undefined) {
      obj.playerId = PlayerId.toJSON(message.playerId);
    }
    if (message.answer !== "") {
      obj.answer = message.answer;
    }
    return obj;
  },

  create(base?: DeepPartial<SubmitAnswerCommand>): SubmitAnswerCommand {
    return SubmitAnswerCommand.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<SubmitAnswerCommand>): SubmitAnswerCommand {
    const message = createBaseSubmitAnswerCommand();
    message.sessionId = (object.sessionId !== undefined && object.sessionId !== null)
      ? GameSessionId.fromPartial(object.sessionId)
      : undefined;
    message.playerId = (object.playerId !== undefined && object.playerId !== null)
      ? PlayerId.fromPartial(object.playerId)
      : undefined;
    message.answer = object.answer ?? "";
    return message;
  },
};

function createBaseRevealHintCommand(): RevealHintCommand {
  return { sessionId: undefined };
}

export const RevealHintCommand = {
  encode(message: RevealHintCommand, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.sessionId !== undefined) {
      GameSessionId.encode(message.sessionId, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RevealHintCommand {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRevealHintCommand();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.sessionId = GameSessionId.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): RevealHintCommand {
    return { sessionId: isSet(object.sessionId) ? GameSessionId.fromJSON(object.sessionId) : undefined };
  },

  toJSON(message: RevealHintCommand): unknown {
    const obj: any = {};
    if (message.sessionId !== undefined) {
      obj.sessionId = GameSessionId.toJSON(message.sessionId);
    }
    return obj;
  },

  create(base?: DeepPartial<RevealHintCommand>): RevealHintCommand {
    return RevealHintCommand.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<RevealHintCommand>): RevealHintCommand {
    const message = createBaseRevealHintCommand();
    message.sessionId = (object.sessionId !== undefined && object.sessionId !== null)
      ? GameSessionId.fromPartial(object.sessionId)
      : undefined;
    return message;
  },
};

function createBaseAnswerAcceptedEvent(): AnswerAcceptedEvent {
  return { sessionId: undefined, playerId: undefined, deltaScore: 0, totalScore: 0 };
}

export const AnswerAcceptedEvent = {
  encode(message: AnswerAcceptedEvent, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.sessionId !== undefined) {
      GameSessionId.encode(message.sessionId, writer.uint32(10).fork()).ldelim();
    }
    if (message.playerId !== undefined) {
      PlayerId.encode(message.playerId, writer.uint32(18).fork()).ldelim();
    }
    if (message.deltaScore !== 0) {
      writer.uint32(24).int32(message.deltaScore);
    }
    if (message.totalScore !== 0) {
      writer.uint32(32).int32(message.totalScore);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AnswerAcceptedEvent {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAnswerAcceptedEvent();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.sessionId = GameSessionId.decode(reader, reader.uint32());
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.playerId = PlayerId.decode(reader, reader.uint32());
          continue;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.deltaScore = reader.int32();
          continue;
        case 4:
          if (tag !== 32) {
            break;
          }

          message.totalScore = reader.int32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): AnswerAcceptedEvent {
    return {
      sessionId: isSet(object.sessionId) ? GameSessionId.fromJSON(object.sessionId) : undefined,
      playerId: isSet(object.playerId) ? PlayerId.fromJSON(object.playerId) : undefined,
      deltaScore: isSet(object.deltaScore) ? globalThis.Number(object.deltaScore) : 0,
      totalScore: isSet(object.totalScore) ? globalThis.Number(object.totalScore) : 0,
    };
  },

  toJSON(message: AnswerAcceptedEvent): unknown {
    const obj: any = {};
    if (message.sessionId !== undefined) {
      obj.sessionId = GameSessionId.toJSON(message.sessionId);
    }
    if (message.playerId !== undefined) {
      obj.playerId = PlayerId.toJSON(message.playerId);
    }
    if (message.deltaScore !== 0) {
      obj.deltaScore = Math.round(message.deltaScore);
    }
    if (message.totalScore !== 0) {
      obj.totalScore = Math.round(message.totalScore);
    }
    return obj;
  },

  create(base?: DeepPartial<AnswerAcceptedEvent>): AnswerAcceptedEvent {
    return AnswerAcceptedEvent.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<AnswerAcceptedEvent>): AnswerAcceptedEvent {
    const message = createBaseAnswerAcceptedEvent();
    message.sessionId = (object.sessionId !== undefined && object.sessionId !== null)
      ? GameSessionId.fromPartial(object.sessionId)
      : undefined;
    message.playerId = (object.playerId !== undefined && object.playerId !== null)
      ? PlayerId.fromPartial(object.playerId)
      : undefined;
    message.deltaScore = object.deltaScore ?? 0;
    message.totalScore = object.totalScore ?? 0;
    return message;
  },
};

function createBaseHintRevealedEvent(): HintRevealedEvent {
  return { sessionId: undefined, hintText: "" };
}

export const HintRevealedEvent = {
  encode(message: HintRevealedEvent, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.sessionId !== undefined) {
      GameSessionId.encode(message.sessionId, writer.uint32(10).fork()).ldelim();
    }
    if (message.hintText !== "") {
      writer.uint32(18).string(message.hintText);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): HintRevealedEvent {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseHintRevealedEvent();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.sessionId = GameSessionId.decode(reader, reader.uint32());
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.hintText = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): HintRevealedEvent {
    return {
      sessionId: isSet(object.sessionId) ? GameSessionId.fromJSON(object.sessionId) : undefined,
      hintText: isSet(object.hintText) ? globalThis.String(object.hintText) : "",
    };
  },

  toJSON(message: HintRevealedEvent): unknown {
    const obj: any = {};
    if (message.sessionId !== undefined) {
      obj.sessionId = GameSessionId.toJSON(message.sessionId);
    }
    if (message.hintText !== "") {
      obj.hintText = message.hintText;
    }
    return obj;
  },

  create(base?: DeepPartial<HintRevealedEvent>): HintRevealedEvent {
    return HintRevealedEvent.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<HintRevealedEvent>): HintRevealedEvent {
    const message = createBaseHintRevealedEvent();
    message.sessionId = (object.sessionId !== undefined && object.sessionId !== null)
      ? GameSessionId.fromPartial(object.sessionId)
      : undefined;
    message.hintText = object.hintText ?? "";
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
