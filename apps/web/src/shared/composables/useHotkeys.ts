import { onMounted, onBeforeUnmount } from 'vue'

export function useHotkeys(map: Record<string, (e: KeyboardEvent) => void>) {
    const handler = (e: KeyboardEvent) => {
        const key = e.key
        if (map[key]) map[key](e)
    }
    onMounted(() => window.addEventListener('keydown', handler))
    onBeforeUnmount(() => window.removeEventListener('keydown', handler))
}
