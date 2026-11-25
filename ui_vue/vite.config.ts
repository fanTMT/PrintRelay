import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite' // 引入插件


export default defineConfig({
  build: {
    rolldownOptions: {
      output: {
        codeSplitting: {
          groups: [
            {
              name: 'vue',
              test: /node_modules[\\/](vue|@vue)[\\/]/,
              priority: 20,
            },
            {
              name: 'vendor',
              test: /node_modules/,
              priority: 10,
              maxSize: 400000, // 超过约 400 kB 自动拆成 vendor, vendor-1, ...
            },
          ],
        },
      },
    },
  },
  plugins: [vue(), tailwindcss()],
})