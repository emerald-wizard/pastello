<template>
  <div class="app">
    <nav class="nav">
      <div class="container nav-inner">
        <router-link to="/" class="brand">Pastel<span class="dot">•</span>Time</router-link>
        <div class="spacer" />
        <router-link to="/tile-mover">Tile Mover</router-link>
      </div>
    </nav>

    <main>
      <router-view />
    </main>
  </div>
</template>

<style scoped>
.nav{
  position: sticky; top: 0; z-index: 10;
  background: linear-gradient(180deg, rgba(255,255,255,0.95), rgba(255,255,255,0.85));
  border-bottom: var(--border-1);
  backdrop-filter: blur(6px);
}
.nav-inner{ display:flex; gap:1rem; align-items:center; padding: .6rem 0; }
.brand{ font-weight: 900; letter-spacing: .2px; }
.dot{ color: var(--color-pink); margin: 0 .15rem; }
.spacer{ flex: 1; }
main{ padding: 1rem 0 2rem; }
</style>
