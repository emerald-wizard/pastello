import { defineStore } from 'pinia';
import { ref } from 'vue';
import { GameSession } from '@gen/runecraftstudios/pastello/game/session/v1/session';

// This is a placeholder for your real User type
interface User {
    id: string;
    username: string;
}

// This is the shape of the reply from our Go "/start-game" endpoint
interface GameTicketResponse {
    ticket: string; // The "Game Ticket" JWT
    session: GameSession; // The initial GameSession data
}

const GO_API_URL = import.meta.env.VITE_API_GO_URL || 'http://localhost:8081';

export const useAuthStore = defineStore('auth', () => {
    // ... (state is the same: loginToken, user) ...

    // ... (login, logout functions are the same) ...

    async function fetchGameTicket(gameType: number): Promise<GameTicketResponse | null> {
        if (!loginToken.value) {
            throw new Error('You must be logged in to start a game.');
        }
        console.log('Asking Go API for a game ticket (MOCKED)...');

        // --- MOCKED RESPONSE ---

        // 1. Define our mock user and session
        const mockUserID = authStore.user?.id || 'user-123';
        const mockSessionID = `game-session-${Math.random().toString(36).substring(2, 9)}`;

        // 2. Create the "magic" dev ticket
        const devTicket = `DEV::${mockUserID}::${mockSessionID}`;

        // 3. Create the mock session data
        const mockSession = GameSession.create({
            id: { value: mockSessionID },
            hostId: { value: mockUserID },
            playerIds: [{ value: mockUserID }], // <-- IMPORTANT for our AuthZ check!
            gameType: gameType,
        });

        const data: GameTicketResponse = {
            ticket: devTicket, // <-- Use the magic string
            session: mockSession,
        };
        // --- END MOCK ---

        console.log(`Received Dev Ticket (${devTicket}) from mock API`);
        return data;
    }

    return {
        // ... (same as before) ...
        fetchGameTicket,
    };
});
