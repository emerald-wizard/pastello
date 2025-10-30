import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
    { path: '/', component: () => import('../pages/Home.vue') },
    { path: '/tile-mover', component: () => import('../features/tile-mover/pages/TileMoverPage.vue') },
]

export const router = createRouter({
    history: createWebHistory(),
    routes,
    scrollBehavior() { return { top: 0 } }
})
