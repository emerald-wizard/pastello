port { GameClient } from '@/api/gameClient'; // Using @ alias
import {
    Envelope,
} from '@gen/runecraftstudios/pastello/web/game/v1/envelope';

// --- Import our NEW stores ---
import { useAuthStore } from '@/app/stores/auth.store';
import { useGameStore } from '@/app/stores/game.store';

// Get the WS URL
const VITE_API_WS_URL = import.meta.env.VITE_API_WS_URL;
if (!VITE_API_WS_URL) {
    throw new Error('Missing VITE_API_WS_URL in .env.local');
}

// Create a single, shared client instance
const client = new GameClient(VITE_API_WS_URL);

// --- This composable is now stateless! ---
// It reads and writes to the Pinia stores.
export function useGameClient() {
    // Get a reference to the stores
    const authStore = useAuthStore();
    const gameStore = useGameStore();

    // Link client events to the gameStore
    client.onOpen = () => {
        gameStore.setConnectionStatus({ isConnected: true, isConnecting: false, error: null });
    };
    client.onClose = () => {
        gameStore.setConnectionStatus({ isConnected: false, isConnecting: false });
        gameStore.clearSession(); // Clear game data on disconnect
    };
    client.onError = (error) => {
        gameStore.setConnectionStatus({
            isConnected: false,
            isConnecting: false,
            error: error?.message || 'A WebSocket error occurred.'
        });
    };

    /**
     * --- MODIFIED connect() ---
     * It now gets the ticket from the authStore.
     */
    const connect = async (ticket: string) => {
        const { isConnected, isConnecting } = gameStore;
        if (!isConnected && !isConnecting) {
            gameStore.setConnectionStatus({ isConnecting: true });
            client.connectAndAuth(ticket);
        }
    };

    const disconnect = () => {
        client.disconnect();
    };

    /**
     * --- MODIFIED sendRequest() ---
     * It no longer updates state here. It just sends.
     * We will update state by listening for broadcast events.
     */
    const sendRequest = async (
        body: Envelope['body'],
    ): Promise<Envelope> => {
        try {
            const reply = await client.sendRequest(body);

            // We could update the gameStore session here if needed,
            // e.g., if the reply contains new session data.
            // if (reply.body?.$case === '...') {
            //   gameStore.setSession(...)
            // }

            return reply;
        } catch (error: any) {
            console.error('Request failed:', error);
            gameStore.setConnectionStatus({ error: error.message });
            throw error;
        }
    };

    return {
        // Note: We no longer return 'state'.
        // Components will get state from the stores.
        connect,
        disconnect,
        sendRequest,
    };
}