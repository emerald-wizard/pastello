import type { Board } from './board'

export interface MovePieceCommand {
    sessionId: string
    playerId: string
    fromX: number; fromY: number
    toX: number; toY: number
}
export type VerifyResult = { ok: true } | { ok: false; reason: string }

const manhattan = (ax: number, ay: number, bx: number, by: number) =>
    Math.abs(ax - bx) + Math.abs(ay - by)

// safe cell access (strict/TS-friendly)
function cell(board: Board, x: number, y: number): number | undefined {
    const row = board[y]
    return row?.[x]
}

export function verifyMove(board: Board, cmd: MovePieceCommand): VerifyResult {
    const N = board.length
    const { fromX, fromY, toX, toY } = cmd

    // integers
    if (![fromX, fromY, toX, toY].every(Number.isInteger)) {
        return { ok: false, reason: 'Coordinates must be integers' }
    }

    // bounds
    if (
        fromX < 0 || fromX >= N || toX < 0 || toX >= N ||
        fromY < 0 || fromY >= N || toY < 0 || toY >= N
    ) {
        return { ok: false, reason: 'Out of bounds' }
    }

    // read via helper; TS now knows these could be undefined, so we guard once
    const fromVal = cell(board, fromX, fromY)
    const toVal = cell(board, toX, toY)
    if (fromVal === undefined || toVal === undefined) {
        return { ok: false, reason: 'Out of bounds' }
    }

    if (fromVal === 0) return { ok: false, reason: 'Cannot move the empty slot' }
    if (toVal !== 0) return { ok: false, reason: 'Destination must be the empty slot' }

    if (manhattan(fromX, fromY, toX, toY) !== 1) {
        return { ok: false, reason: 'Move must be one tile orthogonally' }
    }

    return { ok: true }
}
