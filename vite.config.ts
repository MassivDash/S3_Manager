import postcss from './postcss.config.js';
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tsconfigPaths from 'vite-tsconfig-paths'
import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte(), tsconfigPaths()],
  optimizeDeps: { exclude: ["svelte-navigator"] },
  css:{
    postcss
  }
})