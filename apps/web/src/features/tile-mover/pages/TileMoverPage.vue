<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'

import { useAuthStore } from '@/app/stores/auth.store'
import { useGameStore } from '@/app/stores/game.store'
import { useGameClient } from '@/app/composables/useGameClient'

import TileBoard from '../components/TileBoard.vue'
import { GameType, PlayerId } from '@gen/runecraftstudios/pastello/game/types/v1/types'
import { MovePieceCommand } from '@gen/runecraftstudios/pastello/game/puzzle/v1/commands'

const router = useRouter()
const authStore = useAuthStore()
const gameStore = useGameStore()
const { connect, disconnect, sendGameCommand, sendStartGame } = useGameClient()

const isLoading = ref(false)

const startGame = async () => {
  if (!authStore.user) {
    alert('You must be logged in to play!')
    return
  }

  isLoading.value = true
  gameStore.setConnectionStatus({ error: null })

  try {
    const response = await authStore.fetchGameTicket(GameType.GAME_TYPE_PUZZLE)
    gameStore.setSession(response.session)

    await connect(response.ticket)
    sendStartGame(GameType.GAME_TYPE_PUZZLE)
  } catch (error: any) {
    console.error('Failed to start game:', error)
    gameStore.setConnectionStatus({
      error: error.message,
      isConnecting: false,
      isConnected: false,
    })
  } finally {
    isLoading.value = false
  }
}

const handleMove = (move: {
  fromX: number
  fromY: number
  toX: number
  toY: number
}) => {
  if (!gameStore.session?.id) return

  try {
    const command: MovePieceCommand = {
      sessionId: gameStore.session.id,
      playerId: authStore.user ? PlayerId.create({ value: authStore.user.id }) : undefined,
      fromX: move.fromX,
      fromY: move.fromY,
      toX: move.toX,
      toY: move.toY,
    }

    sendGameCommand({
      $case: 'puzzleMove',
      puzzleMove: command,
    })
  } catch (error: any) {
    console.error('Move failed:', error)
    gameStore.setConnectionStatus({ error: error.message })
  }
}

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
        <h2>Game: {{ gameStore.session?.id?.value ?? 'pending-session' }}</h2>
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
