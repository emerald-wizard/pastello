import { defineStore } from 'pinia';
import { ref } from 'vue';
import { GameSession } from '@gen/runecraftstudios/pastello/game/session/v1/session';

export const useGameStore = defineStore('game', () => {
    // --- State ---
    const session = ref<GameSession | null>(null);
    const isConnecting = ref(false);
    const isConnected = ref(false);
    const error = ref<string | null>(null);

    // --- Actions ---
    function setSession(newSession: GameSession) {
        session.value = newSession;
    }

    function clearSession() {
        session.value = null;
        isConnected.value = false;
        isConnecting.value = false;
    }

    function setConnectionStatus(status: { isConnecting?: boolean, isConnected?: boolean, error?: string | null }) {
        if (status.isConnecting !== undefined) isConnecting.value = status.isConnecting;
        if (status.isConnected !== undefined) isConnected.value = status.isConnected;
        if (status.error !== undefined) error.value = status.error;
    }

    return {
        session,
        isConnecting,
        isConnected,
        error,
        setSession,
        clearSession,
        setConnectionStatus,
    };
});