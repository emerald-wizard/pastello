import { shuffleInPlace } from '@/shared/lib/rand'

export type Board = number[][]

export function makeSolved(size = 4): Board {
    const nums = Array.from({ length: size * size }, (_, i) => i) // 0..15
    nums.push(nums.shift()!) // [1..15,0]
    const grid: Board = []
    for (let y = 0; y < size; y++) grid.push(nums.slice(y * size, (y + 1) * size))
    return grid
}
export function findEmpty(b: Board) {
    for (let y = 0; y < b.length; y++) {
        const row = b[y]
        if (!row) continue                        // row may be undefined

        for (let x = 0; x < row.length; x++) {
            const cell = row[x]                     // number | undefined
            if (cell === 0) return { x, y }         // only true when defined and 0
        }
    }
    return { x: -1, y: -1 }
}

export function shuffleSolvable(board: Board): Board {
    const size = board.length
    const flat: number[] = board.flat()         // still index-safe checks below

    // Move 0 to start (guarded)
    const z = flat.indexOf(0)
    if (z >= 0 && z < flat.length && flat[0] !== undefined && flat[z] !== undefined) {
        const t = flat[0]
        flat[0] = flat[z]
        flat[z] = t
    }

    shuffleInPlace(flat)

    // Count inversions (ignoring 0)
    let inversions = 0
    for (let i = 0; i < flat.length; i++) {
        const n = flat[i]
        if (n === undefined || n === 0) continue
        for (let j = i + 1; j < flat.length; j++) {
            const m = flat[j]
            if (m !== undefined && m !== 0 && m < n) inversions++
        }
    }


    const zeroPos = flat.indexOf(0)
    const zeroRow = Math.floor(zeroPos / size)  // 0-based from top
    const emptyRowFromBottom = size - zeroRow   // 1..size

    // Ensure solvable: if parity is wrong, swap two non-zero neighbors
    if ((inversions + emptyRowFromBottom) % 2 !== 0) {
        for (let i = 0; i < flat.length - 1; i++) {
            const a = flat[i]
            const b = flat[i + 1]
            if (a !== undefined && b !== undefined && a !== 0 && b !== 0) {
                flat[i] = b
                flat[i + 1] = a
                break
            }
        }
    }

    // Rebuild board
    const out: Board = []
    for (let y = 0; y < size; y++) {
        out.push(flat.slice(y * size, (y + 1) * size))
    }
    return out
}


export function isSolved(b: Board): boolean {
    const target = makeSolved(b.length).flat().join(',')
    return b.flat().join(',') === target
}
