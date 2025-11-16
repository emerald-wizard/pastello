/* eslint-disable */
import _m0 from "protobufjs/minimal";

export const protobufPackage = "runecraftstudios.pastello.game.puzzle.v1";

export enum PuzzleDifficulty {
  PUZZLE_DIFFICULTY_UNSPECIFIED = 0,
  PUZZLE_DIFFICULTY_EASY = 1,
  PUZZLE_DIFFICULTY_MEDIUM = 2,
  PUZZLE_DIFFICULTY_HARD = 3,
  UNRECOGNIZED = -1,
}

export function puzzleDifficultyFromJSON(object: any): PuzzleDifficulty {
  switch (object) {
    case 0:
    case "PUZZLE_DIFFICULTY_UNSPECIFIED":
      return PuzzleDifficulty.PUZZLE_DIFFICULTY_UNSPECIFIED;
    case 1:
    case "PUZZLE_DIFFICULTY_EASY":
      return PuzzleDifficulty.PUZZLE_DIFFICULTY_EASY;
    case 2:
    case "PUZZLE_DIFFICULTY_MEDIUM":
      return PuzzleDifficulty.PUZZLE_DIFFICULTY_MEDIUM;
    case 3:
    case "PUZZLE_DIFFICULTY_HARD":
      return PuzzleDifficulty.PUZZLE_DIFFICULTY_HARD;
    case -1:
    case "UNRECOGNIZED":
    default:
      return PuzzleDifficulty.UNRECOGNIZED;
  }
}

export function puzzleDifficultyToJSON(object: PuzzleDifficulty): string {
  switch (object) {
    case PuzzleDifficulty.PUZZLE_DIFFICULTY_UNSPECIFIED:
      return "PUZZLE_DIFFICULTY_UNSPECIFIED";
    case PuzzleDifficulty.PUZZLE_DIFFICULTY_EASY:
      return "PUZZLE_DIFFICULTY_EASY";
    case PuzzleDifficulty.PUZZLE_DIFFICULTY_MEDIUM:
      return "PUZZLE_DIFFICULTY_MEDIUM";
    case PuzzleDifficulty.PUZZLE_DIFFICULTY_HARD:
      return "PUZZLE_DIFFICULTY_HARD";
    case PuzzleDifficulty.UNRECOGNIZED:
    default:
      return "UNRECOGNIZED";
  }
}

export interface PuzzleRules {
  difficulty: PuzzleDifficulty;
  allowHints: boolean;
  timeLimitSeconds: number;
  maxPlayers: number;
}

function createBasePuzzleRules(): PuzzleRules {
  return { difficulty: 0, allowHints: false, timeLimitSeconds: 0, maxPlayers: 0 };
}

export const PuzzleRules = {
  encode(message: PuzzleRules, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.difficulty !== 0) {
      writer.uint32(8).int32(message.difficulty);
    }
    if (message.allowHints === true) {
      writer.uint32(16).bool(message.allowHints);
    }
    if (message.timeLimitSeconds !== 0) {
      writer.uint32(24).uint32(message.timeLimitSeconds);
    }
    if (message.maxPlayers !== 0) {
      writer.uint32(32).uint32(message.maxPlayers);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): PuzzleRules {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBasePuzzleRules();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.difficulty = reader.int32() as any;
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.allowHints = reader.bool();
          continue;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.timeLimitSeconds = reader.uint32();
          continue;
        case 4:
          if (tag !== 32) {
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

  fromJSON(object: any): PuzzleRules {
    return {
      difficulty: isSet(object.difficulty) ? puzzleDifficultyFromJSON(object.difficulty) : 0,
      allowHints: isSet(object.allowHints) ? globalThis.Boolean(object.allowHints) : false,
      timeLimitSeconds: isSet(object.timeLimitSeconds) ? globalThis.Number(object.timeLimitSeconds) : 0,
      maxPlayers: isSet(object.maxPlayers) ? globalThis.Number(object.maxPlayers) : 0,
    };
  },

  toJSON(message: PuzzleRules): unknown {
    const obj: any = {};
    if (message.difficulty !== 0) {
      obj.difficulty = puzzleDifficultyToJSON(message.difficulty);
    }
    if (message.allowHints === true) {
      obj.allowHints = message.allowHints;
    }
    if (message.timeLimitSeconds !== 0) {
      obj.timeLimitSeconds = Math.round(message.timeLimitSeconds);
    }
    if (message.maxPlayers !== 0) {
      obj.maxPlayers = Math.round(message.maxPlayers);
    }
    return obj;
  },

  create(base?: DeepPartial<PuzzleRules>): PuzzleRules {
    return PuzzleRules.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<PuzzleRules>): PuzzleRules {
    const message = createBasePuzzleRules();
    message.difficulty = object.difficulty ?? 0;
    message.allowHints = object.allowHints ?? false;
    message.timeLimitSeconds = object.timeLimitSeconds ?? 0;
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
