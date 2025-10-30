<script setup lang="ts">
defineProps<{ title: string; to: string; description?: string }>()
</script>

<template>
  <router-link :to="to" class="card game-card">
    <div class="content">
      <h3>{{ title }}</h3>
      <p v-if="description">{{ description }}</p>
    </div>
    <span class="cta">Play →</span>
  </router-link>
</template>

<style scoped>
.game-card{
  display:flex; align-items:center; justify-content:space-between;
  padding: 1rem 1.2rem; margin: .5rem 0;
  transition: transform .08s ease;
}
.game-card:hover{ transform: translateY(-1px); }
.content h3{ margin: 0 0 .25rem 0; }
.content p { margin: 0; opacity:.8; }
.cta{ font-weight:700; }
</style>
