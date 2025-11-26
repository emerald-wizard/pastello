import { createApp } from 'vue';
import { createPinia } from 'pinia'; // <-- Import
import App from '../App.vue';
import { router } from './router';

import './styles/globals.css';

const app = createApp(App);
const pinia = createPinia(); // <-- Create

app.use(router);
app.use(pinia); // <-- Use

app.mount('#app');
