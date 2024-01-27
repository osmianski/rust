import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'

const env = loadEnv("", process.cwd(), "");

// https://vitejs.dev/config/
/** @type {import('vite').UserConfig} */
export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        main: './js/app.js',
      }
    },
  },
  plugins: [
    vue()
  ],
  server: {
    port: env.VITE_PORT || 5173,
    strictPort: true,
  }
})
