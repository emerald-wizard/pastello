import { GameClient } from '@/api/gameClient'
import { useGameStore } from '@/app/stores/game.store'
import { GameType } from '@gen/runecraftstudios/pastello/game/types/v1/types'
import { GameCommandEnvelope } from '@gen/runecraftstudios/pastello/web/game/v1/envelope'

const VITE_API_WS_URL = import.meta.env.VITE_API_WS_URL || 'ws://localhost:8080/ws/game'

// Reuse a single client instance for the entire app.
const client = new GameClient(VITE_API_WS_URL)

export function useGameClient() {
  const gameStore = useGameStore()

  client.onOpen = () => {
    gameStore.setConnectionStatus({ isConnected: true, isConnecting: false, error: null })
  }
  client.onClose = () => {
    gameStore.setConnectionStatus({ isConnected: false, isConnecting: false })
    gameStore.clearSession()
  }
  client.onError = (error: Error) => {
    gameStore.setConnectionStatus({
      isConnected: false,
      isConnecting: false,
      error: error?.message || 'A WebSocket error occurred.',
    })
  }

  const connect = async (ticket: string) => {
    if (gameStore.isConnected || gameStore.isConnecting) return
    gameStore.setConnectionStatus({ isConnecting: true, error: null })
    await client.connectAndAuth(ticket)
    gameStore.setConnectionStatus({ isConnected: true, isConnecting: false, error: null })
  }

  const disconnect = () => {
    client.disconnect()
  }

  const sendStartGame = (gameType: GameType) => {
    client.sendStartGame(gameType)
  }

  const sendGameCommand = (command: GameCommandEnvelope['command']) => {
    client.sendGameCommand(command)
  }

  return {
    connect,
    disconnect,
    sendStartGame,
    sendGameCommand,
  }
}
