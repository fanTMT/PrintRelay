import { createApp } from 'vue'
import './style.css'
// @ts-expect-error Vue SFCs are handled by the Vue/Vite tooling.
import App from './App.vue'


createApp(App).mount('#app')
