<script setup lang="ts">
const props = defineProps<{
  n: number
  x: number
  y: number
  disabled?: boolean
}>()

const emit = defineEmits<{
  /** Fired when the tile is activated (clicked). */
  (e: 'activate', payload: { x: number; y: number }): void
}>()

function onClick() {
  if (props.disabled || props.n === 0) return
  emit('activate', { x: props.x, y: props.y })
}
</script>

<template>
  <button
    class="tile"
    :class="{ empty: n === 0 }"
    :disabled="disabled || n === 0"
    @click="onClick"
    :aria-label="n === 0 ? 'Empty' : `Tile ${n}`"
  >
    <span v-if="n !== 0">{{ n }}</span>
  </button>
</template>

<style scoped>
.tile{
  aspect-ratio: 1/1;
  border-radius: var(--radius-lg);
  border: var(--border-1);
  box-shadow: var(--shadow-1);
  background: var(--color-yellow);
  color: var(--color-ink);
  font-weight: 700;
  font-size: 1.25rem;
  display:flex; align-items:center; justify-content:center;
  cursor: pointer;
  transition: transform .06s ease, box-shadow .06s ease, opacity .15s ease;
}
.tile:hover{ transform: translateY(-1px); }
.tile:active{ transform: translateY(1px); }
.tile.empty{
  background: #fff;
  border: 1px dashed rgba(0,0,0,.15);
  box-shadow: none;
  cursor: default;
}
</style>
