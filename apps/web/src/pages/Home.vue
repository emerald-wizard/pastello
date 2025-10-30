<script setup lang="ts">
import GameCard from '@/shared/components/GameCard.vue'
</script>

<template>
  <section class="container">
    <div class="hero card">
      <h1>Pastel Games</h1>
      <p>Pick a game to play. More coming soon.</p>
      <div class="cta">
        <router-link to="/tile-mover" class="btn yellow">Play Tile Mover</router-link>
      </div>
    </div>

    <h2>Games</h2>
    <GameCard
      title="Tile Mover"
      to="/tile-mover"
      description="Classic 15-puzzle with local verification and protobuf-ready seams."
    />
  </section>
</template>

<style scoped>
.hero{ padding: 1.2rem; margin: 1rem 0 1.5rem; }
.cta{ margin-top: .5rem; }
h2{ margin: 1.5rem 0 .5rem; }
</style>
