<template>
  <section class="container">
    <h2>Tile Mover</h2>
    <p>Click a tile next to the empty space to move it. Arrow keys work too.</p>
    <TileBoard />
  </section>
</template>

<script setup lang="ts">
import TileBoard from '../components/TileBoard.vue'
</script>
