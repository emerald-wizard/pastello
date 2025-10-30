<script setup lang="ts">
import { computed, reactive } from 'vue'
import TileTile from './TileTile.vue'
import {
  makeSolved,
  shuffleSolvable,
  findEmpty,
  isSolved,
  type Board
} from '../logic/board'
import {
  verifyMoveCommand,
  type MovePieceCommand
} from '../api/moveVerifierClient'

const SIZE = 4

// --- state ---
type State = {
  board: Board
  lastResult: string
  sessionId: string
  playerId: string
  moves: MovePieceCommand[]
}
const state = reactive<State>({
  board: shuffleSolvable(makeSolved(SIZE)),
  lastResult: '',
  sessionId: 'local-demo-session',
  playerId: 'local-player',
  moves: []
})

const solved  = computed(() => isSolved(state.board))
const canUndo = computed(() => state.moves.length > 0)

// --- helpers ---
function swapTiles(b: Board, ax: number, ay: number, bx: number, by: number): boolean {
  const rowA = b[ay], rowB = b[by]
  if (!rowA || !rowB) return false
  if (rowA[ax] === undefined || rowB[bx] === undefined) return false
  const t = rowA[ax]!
  rowA[ax] = rowB[bx]!
  rowB[bx] = t
  return true
}

// --- actions ---
function clickAt(x: number, y: number) {
  const empty = findEmpty(state.board)
  if (empty.x < 0 || empty.y < 0) return

  const cmd: MovePieceCommand = {
    sessionId: state.sessionId,
    playerId:  state.playerId,
    fromX: x, fromY: y,
    toX: empty.x, toY: empty.y
  }

  verifyMoveCommand(state.board, cmd).then(res => {
    if (!res.ok) { state.lastResult = `❌ ${res.reason}`; return }
    if (!swapTiles(state.board, empty.x, empty.y, x, y)) {
      state.lastResult = '❌ Swap out of bounds'
      return
    }
    state.moves.push(cmd)
    state.lastResult = `✅ (${x},${y}) → (${empty.x},${empty.y})`
  })
}

function shuffle() {
  state.board = shuffleSolvable(makeSolved(SIZE))
  state.moves = []
  state.lastResult = '🔀 Shuffled'
}

function undo() {
  const last = state.moves.pop()
  if (!last) return
  const { fromX, fromY, toX, toY } = last
  if (!swapTiles(state.board, toX, toY, fromX, fromY)) return
  state.lastResult = '↩️ Undo'
}

function onActivate(e: { x: number; y: number }) {
  clickAt(e.x, e.y)
}
</script>

<template>
  <div class="board-wrap card">
    <div class="toolbar">
      <button class="btn pink" @click="shuffle">Shuffle</button>
      <button class="btn green" :disabled="!canUndo" @click="undo">Undo</button>
      <span class="result" :data-ok="state.lastResult.startsWith('✅')">{{ state.lastResult }}</span>
      <span v-if="solved" class="win">🎉 Solved!</span>
    </div>

    <div class="grid" :style="{ '--size': SIZE }">
      <template v-for="(row, y) in state.board" :key="y">
        <TileTile
          v-for="(n, x) in row"
          :key="`${y}-${x}`"
          :n="n"
          :x="x"
          :y="y"
          :disabled="n === 0"
          @activate="onActivate"  
        ></TileTile>
      </template>
    </div>
  </div>
</template>

<style scoped>
.board-wrap{ padding: 1rem; }
.toolbar{ display:flex; gap:.5rem; align-items:center; margin-bottom:.75rem; flex-wrap:wrap; }
.result{ margin-left:.25rem; opacity:.9; font-size:.95rem; }
.win{ margin-left:auto; font-weight:700; }

.grid{
  --size: 4;
  display: grid;
  grid-template-columns: repeat(var(--size), 1fr);
  gap: 10px;
}
</style>
