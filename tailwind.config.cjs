/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./index.html', './src/**/*.{svelte,js,ts}'],
  options: {
    // Generate col-span-1 -> 12
    safelist: [...Array.from({ length: 12. }).fill('').map((_, i) => `grid-cols-${i + 1}`)],
  },
  theme: {
    extend: {
      fontFamily: {
        'roboto': ['"roboto"', 'sans serif']
      }
    },
  },
  plugins: [],
}

