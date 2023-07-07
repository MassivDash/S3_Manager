import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";



export default defineConfig({
  plugins: [svelte({ hot: !process.env.VITEST })],
  test: {
    alias: [{ find: /^svelte$/, replacement: 'svelte/internal' }],
    include: ["src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}"],
    globals: true,
    environment: "jsdom",
    setupFiles: ['./src/test/setup.ts'],
  },
});
