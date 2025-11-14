import {
    Envelope,
} from '@gen/runecraftstudios/pastello/web/game/v1/envelope'; // Path alias
import { v4 as uuidv4 } from 'uuid';

type PendingRequest = {
    // ... (same as before)
    resolve: (value: Envelope) => void;
    reject: (reason?: any) => void;
    timeout: number;
};
const REQUEST_TIMEOUT_MS = 10000;

export class GameClient {
    private ws: WebSocket | null = null;
    private pending = new Map<string, PendingRequest>();
    private url: string;

    public onOpen: (() => void) | null = null;
    public onClose: (() => void) | null = null;
    public onError: ((error: any) => void) | null = null;

    constructor(url: string) {
        this.url = url;
    }

    /**
     * --- MODIFIED ---
     * This now takes the "Game Ticket" and sends it onopen.
     */
    public connectAndAuth(ticket: string) {
        if (this.ws) {
            console.warn('GameClient is already connected or connecting.');
            return;
        }

        console.log(`Connecting to ${this.url}...`);
        this.ws = new WebSocket(this.url);
        this.ws.binaryType = 'arraybuffer';

        this.ws.onopen = () => {
            // --- THIS IS THE NEW AUTH STEP ---
            console.log('WebSocket opened, sending auth ticket...');
            this.ws?.send(ticket); // Send the plain text JWT
            // --- END NEW AUTH STEP ---

            console.log('WebSocket authenticated and connected.');
            this.onOpen?.();
        };

        this.ws.onmessage = (event) => {
            // The first message from the server might be a "welcome"
            // or an "auth error". A real implementation would
            // wait for a confirmation message before firing onOpen.
            // For now, we assume auth success and start listening for Envelopes.
            this.handleMessage(event.data);
        };

        // ... (onerror and onclose are the same as before) ...
        this.ws.onerror = (event) => {
            console.error('WebSocket error:', event);
            this.onError?.(event);
        };

        this.ws.onclose = () => {
            console.log('WebSocket disconnected.');
            this.pending.forEach((req) => req.reject(new Error('WebSocket disconnected')));
            this.pending.clear();
            this.ws = null;
            this.onClose?.();
        };
    }

    public disconnect() {
        this.ws?.close();
    }

    // ... (sendRequest and handleMessage are UNCHANGED) ...
    public async sendRequest(
        body: Envelope['body'],
    ): Promise<Envelope> {
        if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
            return Promise.reject(new Error('WebSocket is not connected.'));
        }

        const correlationId = uuidv4();
        const request = Envelope.create({
            correlationId,
            body,
        });

        const payload = Envelope.encode(request).finish();

        return new Promise((resolve, reject) => {
            const timeout = window.setTimeout(() => {
                this.pending.delete(correlationId);
                reject(new Error(`Request ${correlationId} timed out.`));
            }, REQUEST_TIMEOUT_MS);
            this.pending.set(correlationId, { resolve, reject, timeout });
            this.ws?.send(payload);
        });
    }

    private handleMessage(data: ArrayBuffer) {
        try {
            const envelope = Envelope.decode(new Uint8Array(data));
            const corrId = envelope.correlationId;

            if (!corrId) {
                console.warn('Received message without correlation_id:', envelope);
                return;
            }

            const pending = this.pending.get(corrId);
            if (!pending) {
                console.warn(`Received unknown correlation_id: ${corrId}`);
                return;
            }

            clearTimeout(pending.timeout);
            this.pending.delete(corrId);

            if (envelope.body?.$case === 'error') {
                pending.reject(new Error(envelope.body.error.message));
            } else {
                pending.resolve(envelope);
            }

        } catch (error) {
            console.error('Failed to decode message:', error);
        }
    }
}