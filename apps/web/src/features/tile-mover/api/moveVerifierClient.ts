// Adapter seam. Today just re-exports local verify; later call your protobuf RPC here.
import type { Board } from '../logic/board'
import { verifyMove, type MovePieceCommand, type VerifyResult } from '../logic/verifyLocal'

export async function verifyMoveCommand(board: Board, cmd: MovePieceCommand): Promise<VerifyResult> {
    // later: marshal cmd -> protobuf, call service, unmarshal result
    return verifyMove(board, cmd)
}

export type { MovePieceCommand, VerifyResult }
