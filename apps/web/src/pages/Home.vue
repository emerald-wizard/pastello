<script setup lang="ts">
import GameCard from '@/shared/components/GameCard.vue';
import { useAuthStore } from '@/app/stores/auth.store'; // <-- 1. Import the store

// 2. Get the store instance
const authStore = useAuthStore();
</script>

<template>
  <section class="container">
    <div class="hero card">
      <h1>Pastel Games</h1>
      <p>Pick a game to play. More coming soon.</p>
      
      <div class="auth-box">
        <div v-if="!authStore.user" class="cta">
          <button @click="authStore.login('mockUser', 'mockPass')" class="btn green">
            Mock Login
          </button>
        </div>
        <div v-else class="welcome-box">
          <span>Welcome, <strong>{{ authStore.user.username }}</strong>!</span>
          <button @click="authStore.logout()" class="btn-link">
            (Logout)
          </button>
        </div>
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
.hero {
  padding: 1.2rem;
  margin: 1rem 0 1.5rem;
}
.cta {
  margin-top: 0.5rem;
}
h2 {
  margin: 1.5rem 0 0.5rem;
}

/* New styles for the auth box */
.auth-box {
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--color-border);
}
.welcome-box {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
.btn-link {
  background: none;
  border: none;
  color: var(--color-link);
  cursor: pointer;
  padding: 0;
  font-size: 0.9rem;
}
.btn-link:hover {
  text-decoration: underline;
}
</style>