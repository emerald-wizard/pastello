<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'

// 1. Import all our new tools
import { useAuthStore } from '@/app/stores/auth.store'
import { useGameStore } in '@/app/stores/game.store'
import { useGameClient } from '@/app/composables/useGameClient'

// Import components and types
import TileBoard from '../components/TileBoard.vue'
import { GameType } from '@gen/runecraftstudios/pastello/game/types/v1/types'
import { PuzzleMovePieceCommand } from '@gen/runecraftstudios/pastello/game/puzzle/v1/commands'
import { PlayerId } from '@gen/runecraftstudios/pastello/game/types/v1/types'
import { GameSessionId } from '@gen/runecraftstudios/pastello/game/session/v1/session'

// 2. Get instances of everything
const router = useRouter()
const authStore = useAuthStore()
const gameStore = useGameStore()
const { connect, disconnect, sendRequest } = useGameClient()

const isLoading = ref(false)

// 3. This is the new "Start Game" flow
const startGame = async () => {
  if (!authStore.user) {
    // This shouldn't happen if the UI is right, but good to check
    alert('You must be logged in to play!')
    return
  }

  isLoading.value = true
  gameStore.setConnectionStatus({ error: null })

  try {
    // 1. Call our (mocked) authStore to get the DEV:: token
    const response = await authStore.fetchGameTicket(GameType.PUZZLE)
    if (!response) throw new Error('Failed to get game ticket from server.')

    // 2. Save the mock session data in our game store
    gameStore.setSession(response.session)

    // 3. Connect to Rust WS with the new DEV:: ticket
    await connect(response.ticket)

    // We are now connected and authenticated!
  } catch (error: any) {
    console.error('Failed to start game:', error)
    gameStore.setConnectionStatus({ error: error.message })
  } finally {
    isLoading.value = false
  }
}

// 4. This is the new "Handle Move" flow
const handleMove = async (move: {
  fromX: number
  fromY: number
  toX: number
  toY: number
}) => {
  // We read the session ID from the Pinia store
  if (!gameStore.session?.id) return

  try {
    const command = PuzzleMovePieceCommand.create({
      sessionId: gameStore.session.id,
      // The backend will get the player ID from the token,
      // but we can send it anyway.
      playerId: authStore.user ? PlayerId.create({ value: authStore.user.id }) : undefined,
      fromX: move.fromX,
      fromY: move.fromY,
      toX: move.toX,
      toY: move.toY,
    })

    const reply = await sendRequest({
      $case: 'puzzleMovePiece',
      puzzleMovePiece: command,
    })

    if (reply.body?.$case === 'puzzlePieceMoved') {
      console.log('Move confirmed by server:', reply.body.puzzlePieceMoved)
      // The local board logic in TileBoard.vue will update
      // the visual state. The server has just confirmed it's a valid move.
    } else if (reply.body?.$case === 'error') {
      // Handle a server-side error (e.g., invalid move)
      console.error('Move rejected:', reply.body.error.message)
      gameStore.setConnectionStatus({ error: reply.body.error.message })
    }
  } catch (error: any) {
    console.error('Move failed:', error)
    gameStore.setConnectionStatus({ error: error.message })
  }
}

// 5. Clean up the connection when we leave the page
const leaveGame = () => {
  disconnect()
  gameStore.clearSession()
  router.push('/')
}

onUnmounted(() => {
  disconnect()
  gameStore.clearSession()
})
</script>

<template>
  <section class="container">
    <button @click="leaveGame" class="btn-link">← Back to Home</button>

    <div class="game-container">
      <div v-if="!gameStore.session">
        <h2>Tile Mover</h2>
        <p>A classic 15-puzzle game.</p>
        <button @click="startGame" :disabled="isLoading || !authStore.user">
          {{ isLoading ? 'Starting...' : 'Start Game' }}
        </button>
        <p v-if="!authStore.user" class="error-text">
          You must be "logged in" on the home page to play.
        </p>
      </div>

      <div v-else>
        <h2>Game: {{ gameStore.session.id.value }}</h2>
        <p v-if="gameStore.isConnecting">Connecting...</p>
        <p v-if="gameStore.isConnected">
          Connected as: {{ authStore.user?.username }}
        </p>

        <TileBoard @move="handleMove" />
      </div>

      <p v-if="gameStore.error" class="error-text">
        Error: {{ gameStore.error }}
      </p>
    </div>
  </section>
</template>

<style scoped>
.game-container {
  margin-top: 1.5rem;
}
.btn-link {
  background: none;
  border: none;
  color: var(--color-link);
  cursor: pointer;
  padding: 0;
  font-size: 1rem;
}
.btn-link:hover {
  text-decoration: underline;
}
.error-text {
  color: var(--color-error);
  margin-top: 1rem;
}
</style>