import {
  ClientEnvelope,
  GameCommandEnvelope,
  ServerEnvelope,
  StartGameCommand,
} from '@gen/runecraftstudios/pastello/web/game/v1/envelope'
import { GameType } from '@gen/runecraftstudios/pastello/game/types/v1/types'

const AUTH_TIMEOUT_MS = 5000

export class GameClient {
  private ws: WebSocket | null = null
  private url: string
  private authTimeout: number | null = null

  public onOpen: (() => void) | null = null
  public onClose: (() => void) | null = null
  public onError: ((error: Error) => void) | null = null
  public onServerEnvelope: ((payload: ServerEnvelope) => void) | null = null

  constructor(url: string) {
    this.url = url
  }

  /**
   * Connects to the Rust WebSocket and sends the stub ticket as
   * the very first text frame. Resolves once AUTH_SUCCESS is received.
   */
  public connectAndAuth(ticket: string): Promise<void> {
    if (this.ws) {
      return Promise.resolve()
    }

    return new Promise((resolve, reject) => {
      const socket = new WebSocket(this.url)
      this.ws = socket
      socket.binaryType = 'arraybuffer'

      const fail = (error: Error) => {
        this.clearAuthTimeout()
        if (this.ws === socket) this.ws = null
        this.onError?.(error)
        reject(error)
        socket.close()
      }

      socket.onopen = () => {
        socket.send(ticket)
        this.authTimeout = window.setTimeout(() => {
          fail(new Error('Auth handshake timed out'))
        }, AUTH_TIMEOUT_MS)
      }

      socket.onmessage = (event) => {
        if (typeof event.data === 'string') {
          if (event.data === 'AUTH_SUCCESS') {
            this.clearAuthTimeout()
            this.onOpen?.()
            resolve()
            return
          }
          if (event.data === 'AUTH_FAILED') {
            fail(new Error('Authentication failed'))
            return
          }
          // Unknown text frames are ignored for now
          return
        }

        this.handleBinary(event.data as ArrayBuffer)
      }

      socket.onerror = () => fail(new Error('WebSocket error'))

      socket.onclose = () => {
        this.clearAuthTimeout()
        if (this.ws === socket) this.ws = null
        this.onClose?.()
      }
    })
  }

  public disconnect() {
    this.clearAuthTimeout()
    this.ws?.close()
    this.ws = null
  }

  public sendStartGame(gameType: GameType) {
    const start = StartGameCommand.create({ gameType })
    const envelope = ClientEnvelope.create({
      message: { $case: 'startGame', startGame: start },
    })
    this.sendEnvelope(envelope)
  }

  public sendGameCommand(command: GameCommandEnvelope['command']) {
    const envelope = ClientEnvelope.create({
      message: { $case: 'gameCommand', gameCommand: { command } },
    })
    this.sendEnvelope(envelope)
  }

  private sendEnvelope(envelope: ClientEnvelope) {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      throw new Error('WebSocket is not connected.')
    }
    const payload = ClientEnvelope.encode(envelope).finish()
    this.ws.send(payload)
  }

  private handleBinary(data: ArrayBuffer) {
    try {
      const decoded = ServerEnvelope.decode(new Uint8Array(data))
      this.onServerEnvelope?.(decoded)
    } catch (error) {
      console.error('Failed to decode server envelope:', error)
    }
  }

  private clearAuthTimeout() {
    if (this.authTimeout !== null) {
      clearTimeout(this.authTimeout)
      this.authTimeout = null
    }
  }
}
