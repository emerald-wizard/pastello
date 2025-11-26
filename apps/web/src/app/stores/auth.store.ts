import { defineStore } from 'pinia'
import { ref } from 'vue'
import { GameSession } from '@gen/runecraftstudios/pastello/game/session/v1/session'
import { GameType, PlayerId } from '@gen/runecraftstudios/pastello/game/types/v1/types'

type User = {
  id: string
  username: string
}

type GameTicketResponse = {
  ticket: string
  session: GameSession
}

const STUB_USER_ID = 'stub-user-123'
const STUB_SESSION_ID = 'stub-game-session-id'

export const useAuthStore = defineStore('auth', () => {
  const loginToken = ref<string | null>(null)
  const user = ref<User | null>(null)

  function login(username: string, _password?: string) {
    user.value = { id: STUB_USER_ID, username }
    // Stub authenticator accepts any token; we keep the DEV:: format for clarity.
    loginToken.value = `DEV::${STUB_USER_ID}::${STUB_SESSION_ID}`
  }

  function logout() {
    user.value = null
    loginToken.value = null
  }

  async function fetchGameTicket(gameType: GameType): Promise<GameTicketResponse> {
    if (!loginToken.value || !user.value) {
      throw new Error('You must be logged in to start a game.')
    }

    const mockSession = GameSession.create({
      id: { value: STUB_SESSION_ID },
      playerIds: [PlayerId.create({ value: user.value.id })],
      gameType,
      // status/ruleset omitted and fall back to defaults in the generated helpers
    })

    return {
      ticket: loginToken.value,
      session: mockSession,
    }
  }

  return {
    user,
    loginToken,
    login,
    logout,
    fetchGameTicket,
  }
})
