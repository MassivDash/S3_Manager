var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __commonJS = (cb, mod) => function __require() {
  return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target, mod));

// tailwind.config.cjs
var require_tailwind_config = __commonJS({
  "tailwind.config.cjs"(exports, module) {
    module.exports = {
      content: ["./index.html", "./src/**/*.{svelte,js,ts}"],
      options: {
        safelist: [...Array.from({ length: 12 }).fill("").map((_, i) => `columns-${i + 1}`)]
      },
      theme: {
        extend: {
          fontFamily: {
            "roboto": ['"roboto"', "sans serif"]
          }
        }
      },
      plugins: []
    };
  }
});

// postcss.config.js
var import_tailwind_config = __toESM(require_tailwind_config(), 1);
import tailwind from "tailwindcss";
import autoprefixer from "autoprefixer";
var postcss_config_default = {
  plugins: [tailwind(import_tailwind_config.default), autoprefixer]
};

// vite.config.ts
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tsconfigPaths from "vite-tsconfig-paths";
import { defineConfig } from "vite";
var vite_config_default = defineConfig({
  plugins: [svelte(), tsconfigPaths()],
  optimizeDeps: { exclude: ["svelte-navigator"] },
  css: {
    postcss: postcss_config_default
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidGFpbHdpbmQuY29uZmlnLmNqcyIsICJwb3N0Y3NzLmNvbmZpZy5qcyIsICJ2aXRlLmNvbmZpZy50cyJdLAogICJzb3VyY2VzQ29udGVudCI6IFsiLyoqIEB0eXBlIHtpbXBvcnQoJ3RhaWx3aW5kY3NzJykuQ29uZmlnfSAqL1xubW9kdWxlLmV4cG9ydHMgPSB7XG4gIGNvbnRlbnQ6IFsnLi9pbmRleC5odG1sJywgJy4vc3JjLyoqLyoue3N2ZWx0ZSxqcyx0c30nXSxcbiAgb3B0aW9uczoge1xuICAgIC8vIEdlbmVyYXRlIGNvbC1zcGFuLTEgLT4gMTJcbiAgICBzYWZlbGlzdDogWy4uLkFycmF5LmZyb20oeyBsZW5ndGg6IDEyLiB9KS5maWxsKCcnKS5tYXAoKF8sIGkpID0+IGBjb2x1bW5zLSR7aSArIDF9YCldLFxuICB9LFxuICB0aGVtZToge1xuICAgIGV4dGVuZDoge1xuICAgICAgZm9udEZhbWlseToge1xuICAgICAgICAncm9ib3RvJzogWydcInJvYm90b1wiJywgJ3NhbnMgc2VyaWYnXVxuICAgICAgfVxuICAgIH0sXG4gIH0sXG4gIHBsdWdpbnM6IFtdLFxufVxuXG4iLCAiaW1wb3J0IHRhaWx3aW5kIGZyb20gJ3RhaWx3aW5kY3NzJztcbmltcG9ydCBhdXRvcHJlZml4ZXIgZnJvbSAnYXV0b3ByZWZpeGVyJztcbmltcG9ydCB0YWlsd2luZENvbmZpZyBmcm9tICcuL3RhaWx3aW5kLmNvbmZpZy5janMnO1xuXG5leHBvcnQgZGVmYXVsdCB7XG4gIHBsdWdpbnM6IFt0YWlsd2luZCh0YWlsd2luZENvbmZpZyksIGF1dG9wcmVmaXhlcl0sXG59IiwgImltcG9ydCBwb3N0Y3NzIGZyb20gJy4vcG9zdGNzcy5jb25maWcuanMnO1xuaW1wb3J0IHsgc3ZlbHRlIH0gZnJvbSAnQHN2ZWx0ZWpzL3ZpdGUtcGx1Z2luLXN2ZWx0ZSdcbmltcG9ydCB0c2NvbmZpZ1BhdGhzIGZyb20gJ3ZpdGUtdHNjb25maWctcGF0aHMnXG5pbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tICd2aXRlJ1xuXG4vLyBodHRwczovL3ZpdGVqcy5kZXYvY29uZmlnL1xuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKHtcbiAgcGx1Z2luczogW3N2ZWx0ZSgpLCB0c2NvbmZpZ1BhdGhzKCldLFxuICBvcHRpbWl6ZURlcHM6IHsgZXhjbHVkZTogW1wic3ZlbHRlLW5hdmlnYXRvclwiXSB9LFxuICBjc3M6e1xuICAgIHBvc3Rjc3NcbiAgfVxufSkiXSwKICAibWFwcGluZ3MiOiAiOzs7Ozs7Ozs7Ozs7Ozs7Ozs7OztBQUFBO0FBQUE7QUFDQSxXQUFPLFVBQVU7QUFBQSxNQUNmLFNBQVMsQ0FBQyxnQkFBZ0IsMkJBQTJCO0FBQUEsTUFDckQsU0FBUztBQUFBLFFBRVAsVUFBVSxDQUFDLEdBQUcsTUFBTSxLQUFLLEVBQUUsUUFBUSxHQUFJLENBQUMsRUFBRSxLQUFLLEVBQUUsRUFBRSxJQUFJLENBQUMsR0FBRyxNQUFNLFdBQVcsSUFBSSxHQUFHLENBQUM7QUFBQSxNQUN0RjtBQUFBLE1BQ0EsT0FBTztBQUFBLFFBQ0wsUUFBUTtBQUFBLFVBQ04sWUFBWTtBQUFBLFlBQ1YsVUFBVSxDQUFDLFlBQVksWUFBWTtBQUFBLFVBQ3JDO0FBQUEsUUFDRjtBQUFBLE1BQ0Y7QUFBQSxNQUNBLFNBQVMsQ0FBQztBQUFBLElBQ1o7QUFBQTtBQUFBOzs7QUNiQSw2QkFBMkI7QUFGM0I7QUFDQTtBQUdBLElBQU8seUJBQVE7QUFBQSxFQUNiLFNBQVMsQ0FBQyxTQUFTLDhCQUFjLEdBQUcsWUFBWTtBQUNsRDs7O0FDTEE7QUFDQTtBQUNBO0FBR0EsSUFBTyxzQkFBUSxhQUFhO0FBQUEsRUFDMUIsU0FBUyxDQUFDLE9BQU8sR0FBRyxjQUFjLENBQUM7QUFBQSxFQUNuQyxjQUFjLEVBQUUsU0FBUyxDQUFDLGtCQUFrQixFQUFFO0FBQUEsRUFDOUMsS0FBSTtBQUFBLElBQ0Y7QUFBQSxFQUNGO0FBQ0YsQ0FBQzsiLAogICJuYW1lcyI6IFtdCn0K
