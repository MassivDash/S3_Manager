import postcss from './postcss.config.js';
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tsconfigPaths from 'vite-tsconfig-paths'
import { defineConfig } from 'vite'
import { viteStaticCopy } from 'vite-plugin-static-copy';
import { normalizePath } from 'vite';
import { resolve } from 'path';
// https://vitejs.dev/config/


export default defineConfig({
  plugins: [tsconfigPaths(), svelte(), viteStaticCopy({
    targets: [
      {
        src: normalizePath(resolve(__dirname, './splashscreen/index.html')),
        dest:  'splashscreen',
      },
      {
        src: normalizePath(resolve(__dirname, './splashscreen/style.css')),
        dest:  'splashscreen',
      },
    ]
  })],
  server: {
    port: 3000,
  },
  optimizeDeps: { exclude: ["svelte-navigator"] },
  css:{
    postcss
  }
})