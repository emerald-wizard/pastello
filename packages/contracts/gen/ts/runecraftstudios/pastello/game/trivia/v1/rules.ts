/* eslint-disable */
import _m0 from "protobufjs/minimal";

export const protobufPackage = "runecraftstudios.pastello.game.trivia.v1";

export enum TriviaCategory {
  TRIVIA_CATEGORY_UNSPECIFIED = 0,
  TRIVIA_CATEGORY_GENERAL_KNOWLEDGE = 1,
  TRIVIA_CATEGORY_SCIENCE = 2,
  TRIVIA_CATEGORY_HISTORY = 3,
  UNRECOGNIZED = -1,
}

export function triviaCategoryFromJSON(object: any): TriviaCategory {
  switch (object) {
    case 0:
    case "TRIVIA_CATEGORY_UNSPECIFIED":
      return TriviaCategory.TRIVIA_CATEGORY_UNSPECIFIED;
    case 1:
    case "TRIVIA_CATEGORY_GENERAL_KNOWLEDGE":
      return TriviaCategory.TRIVIA_CATEGORY_GENERAL_KNOWLEDGE;
    case 2:
    case "TRIVIA_CATEGORY_SCIENCE":
      return TriviaCategory.TRIVIA_CATEGORY_SCIENCE;
    case 3:
    case "TRIVIA_CATEGORY_HISTORY":
      return TriviaCategory.TRIVIA_CATEGORY_HISTORY;
    case -1:
    case "UNRECOGNIZED":
    default:
      return TriviaCategory.UNRECOGNIZED;
  }
}

export function triviaCategoryToJSON(object: TriviaCategory): string {
  switch (object) {
    case TriviaCategory.TRIVIA_CATEGORY_UNSPECIFIED:
      return "TRIVIA_CATEGORY_UNSPECIFIED";
    case TriviaCategory.TRIVIA_CATEGORY_GENERAL_KNOWLEDGE:
      return "TRIVIA_CATEGORY_GENERAL_KNOWLEDGE";
    case TriviaCategory.TRIVIA_CATEGORY_SCIENCE:
      return "TRIVIA_CATEGORY_SCIENCE";
    case TriviaCategory.TRIVIA_CATEGORY_HISTORY:
      return "TRIVIA_CATEGORY_HISTORY";
    case TriviaCategory.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface TriviaRules {
  numQuestions: number;
  secondsPerQuestion: number;
  negativeMarking: boolean;
  categories: TriviaCategory[];
  maxPlayers: number;
}

function createBaseTriviaRules(): TriviaRules {
  return { numQuestions: 0, secondsPerQuestion: 0, negativeMarking: false, categories: [], maxPlayers: 0 };
}

export const TriviaRules = {
  encode(message: TriviaRules, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.numQuestions !== 0) {
      writer.uint32(8).uint32(message.numQuestions);
    }
    if (message.secondsPerQuestion !== 0) {
      writer.uint32(16).uint32(message.secondsPerQuestion);
    }
    if (message.negativeMarking === true) {
      writer.uint32(24).bool(message.negativeMarking);
    }
    writer.uint32(34).fork();
    for (const v of message.categories) {
      writer.int32(v);
    }
    writer.ldelim();
    if (message.maxPlayers !== 0) {
      writer.uint32(40).uint32(message.maxPlayers);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TriviaRules {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTriviaRules();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.numQuestions = reader.uint32();
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.secondsPerQuestion = reader.uint32();
          continue;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.negativeMarking = reader.bool();
          continue;
        case 4:
          if (tag === 32) {
            message.categories.push(reader.int32() as any);

            continue;
          }

          if (tag === 34) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.categories.push(reader.int32() as any);
            }

            continue;
          }

          break;
        case 5:
          if (tag !== 40) {
            break;
          }

          message.maxPlayers = reader.uint32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): TriviaRules {
    return {
      numQuestions: isSet(object.numQuestions) ? globalThis.Number(object.numQuestions) : 0,
      secondsPerQuestion: isSet(object.secondsPerQuestion) ? globalThis.Number(object.secondsPerQuestion) : 0,
      negativeMarking: isSet(object.negativeMarking) ? globalThis.Boolean(object.negativeMarking) : false,
      categories: globalThis.Array.isArray(object?.categories)
        ? object.categories.map((e: any) => triviaCategoryFromJSON(e))
        : [],
      maxPlayers: isSet(object.maxPlayers) ? globalThis.Number(object.maxPlayers) : 0,
    };
  },

  toJSON(message: TriviaRules): unknown {
    const obj: any = {};
    if (message.numQuestions !== 0) {
      obj.numQuestions = Math.round(message.numQuestions);
    }
    if (message.secondsPerQuestion !== 0) {
      obj.secondsPerQuestion = Math.round(message.secondsPerQuestion);
    }
    if (message.negativeMarking === true) {
      obj.negativeMarking = message.negativeMarking;
    }
    if (message.categories?.length) {
      obj.categories = message.categories.map((e) => triviaCategoryToJSON(e));
    }
    if (message.maxPlayers !== 0) {
      obj.maxPlayers = Math.round(message.maxPlayers);
    }
    return obj;
  },

  create(base?: DeepPartial<TriviaRules>): TriviaRules {
    return TriviaRules.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<TriviaRules>): TriviaRules {
    const message = createBaseTriviaRules();
    message.numQuestions = object.numQuestions ?? 0;
    message.secondsPerQuestion = object.secondsPerQuestion ?? 0;
    message.negativeMarking = object.negativeMarking ?? false;
    message.categories = object.categories?.map((e) => e) || [];
    message.maxPlayers = object.maxPlayers ?? 0;
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
