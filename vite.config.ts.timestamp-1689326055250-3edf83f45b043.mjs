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
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));

// tailwind.config.cjs
var require_tailwind_config = __commonJS({
  "tailwind.config.cjs"(exports, module) {
    module.exports = {
      content: ["./index.html", "./src/**/*.{svelte,js,ts}"],
      options: {
        // Generate col-span-1 -> 12
        safelist: [...Array.from({ length: 12 }).fill("").map((_, i) => `columns-${i + 1}`)]
      },
      theme: {
        extend: {
          fontFamily: {
            "roboto": ['"Roboto"', "sans serif"],
            "audiowide": ['"Audiowide"', "cursive"]
          }
        }
      },
      plugins: []
    };
  }
});

// postcss.config.js
var import_tailwind_config = __toESM(require_tailwind_config(), 1);
import tailwind from "file:///media/spaceghost/data/git/S3_Manager/node_modules/tailwindcss/lib/index.js";
import autoprefixer from "file:///media/spaceghost/data/git/S3_Manager/node_modules/autoprefixer/lib/autoprefixer.js";
var postcss_config_default = {
  plugins: [tailwind(import_tailwind_config.default), autoprefixer]
};

// vite.config.ts
import { svelte } from "file:///media/spaceghost/data/git/S3_Manager/node_modules/@sveltejs/vite-plugin-svelte/src/index.js";
import tsconfigPaths from "file:///media/spaceghost/data/git/S3_Manager/node_modules/vite-tsconfig-paths/dist/index.mjs";
import { defineConfig } from "file:///media/spaceghost/data/git/S3_Manager/node_modules/vite/dist/node/index.js";
import { viteStaticCopy } from "file:///media/spaceghost/data/git/S3_Manager/node_modules/vite-plugin-static-copy/dist/index.js";
import { normalizePath } from "file:///media/spaceghost/data/git/S3_Manager/node_modules/vite/dist/node/index.js";
import { resolve } from "path";
var __vite_injected_original_dirname = "/media/spaceghost/data/git/S3_Manager";
var vite_config_default = defineConfig({
  plugins: [tsconfigPaths(), svelte(), viteStaticCopy({
    targets: [
      {
        src: normalizePath(resolve(__vite_injected_original_dirname, "./splashscreen/index.html")),
        dest: "splashscreen"
      },
      {
        src: normalizePath(resolve(__vite_injected_original_dirname, "./splashscreen/style.css")),
        dest: "splashscreen"
      }
    ]
  })],
  server: {
    port: 3e3
  },
  optimizeDeps: { exclude: ["svelte-navigator"] },
  css: {
    postcss: postcss_config_default
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidGFpbHdpbmQuY29uZmlnLmNqcyIsICJwb3N0Y3NzLmNvbmZpZy5qcyIsICJ2aXRlLmNvbmZpZy50cyJdLAogICJzb3VyY2VzQ29udGVudCI6IFsiY29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2Rpcm5hbWUgPSBcIi9tZWRpYS9zcGFjZWdob3N0L2RhdGEvZ2l0L1MzX01hbmFnZXJcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIi9tZWRpYS9zcGFjZWdob3N0L2RhdGEvZ2l0L1MzX01hbmFnZXIvdGFpbHdpbmQuY29uZmlnLmNqc1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vbWVkaWEvc3BhY2VnaG9zdC9kYXRhL2dpdC9TM19NYW5hZ2VyL3RhaWx3aW5kLmNvbmZpZy5janNcIjsvKiogQHR5cGUge2ltcG9ydCgndGFpbHdpbmRjc3MnKS5Db25maWd9ICovXG5tb2R1bGUuZXhwb3J0cyA9IHtcbiAgY29udGVudDogWycuL2luZGV4Lmh0bWwnLCAnLi9zcmMvKiovKi57c3ZlbHRlLGpzLHRzfSddLFxuICBvcHRpb25zOiB7XG4gICAgLy8gR2VuZXJhdGUgY29sLXNwYW4tMSAtPiAxMlxuICAgIHNhZmVsaXN0OiBbLi4uQXJyYXkuZnJvbSh7IGxlbmd0aDogMTIuIH0pLmZpbGwoJycpLm1hcCgoXywgaSkgPT4gYGNvbHVtbnMtJHtpICsgMX1gKV0sXG4gIH0sXG4gIHRoZW1lOiB7XG4gICAgZXh0ZW5kOiB7XG4gICAgICBmb250RmFtaWx5OiB7XG4gICAgICAgICdyb2JvdG8nOiBbJ1wiUm9ib3RvXCInLCAnc2FucyBzZXJpZiddLFxuICAgICAgICAnYXVkaW93aWRlJzogWydcIkF1ZGlvd2lkZVwiJyAsICdjdXJzaXZlJ10gXG4gICAgICB9XG4gICAgfSxcbiAgfSxcbiAgcGx1Z2luczogW10sXG59XG5cbiIsICJjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZGlybmFtZSA9IFwiL21lZGlhL3NwYWNlZ2hvc3QvZGF0YS9naXQvUzNfTWFuYWdlclwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiL21lZGlhL3NwYWNlZ2hvc3QvZGF0YS9naXQvUzNfTWFuYWdlci9wb3N0Y3NzLmNvbmZpZy5qc1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vbWVkaWEvc3BhY2VnaG9zdC9kYXRhL2dpdC9TM19NYW5hZ2VyL3Bvc3Rjc3MuY29uZmlnLmpzXCI7aW1wb3J0IHRhaWx3aW5kIGZyb20gJ3RhaWx3aW5kY3NzJztcbmltcG9ydCBhdXRvcHJlZml4ZXIgZnJvbSAnYXV0b3ByZWZpeGVyJztcbmltcG9ydCB0YWlsd2luZENvbmZpZyBmcm9tICcuL3RhaWx3aW5kLmNvbmZpZy5janMnO1xuXG5leHBvcnQgZGVmYXVsdCB7XG4gIHBsdWdpbnM6IFt0YWlsd2luZCh0YWlsd2luZENvbmZpZyksIGF1dG9wcmVmaXhlcl0sXG59IiwgImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvbWVkaWEvc3BhY2VnaG9zdC9kYXRhL2dpdC9TM19NYW5hZ2VyXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCIvbWVkaWEvc3BhY2VnaG9zdC9kYXRhL2dpdC9TM19NYW5hZ2VyL3ZpdGUuY29uZmlnLnRzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9tZWRpYS9zcGFjZWdob3N0L2RhdGEvZ2l0L1MzX01hbmFnZXIvdml0ZS5jb25maWcudHNcIjtpbXBvcnQgcG9zdGNzcyBmcm9tICcuL3Bvc3Rjc3MuY29uZmlnLmpzJztcbmltcG9ydCB7IHN2ZWx0ZSB9IGZyb20gJ0BzdmVsdGVqcy92aXRlLXBsdWdpbi1zdmVsdGUnXG5pbXBvcnQgdHNjb25maWdQYXRocyBmcm9tICd2aXRlLXRzY29uZmlnLXBhdGhzJ1xuaW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSAndml0ZSdcbmltcG9ydCB7IHZpdGVTdGF0aWNDb3B5IH0gZnJvbSAndml0ZS1wbHVnaW4tc3RhdGljLWNvcHknO1xuaW1wb3J0IHsgbm9ybWFsaXplUGF0aCB9IGZyb20gJ3ZpdGUnO1xuaW1wb3J0IHsgcmVzb2x2ZSB9IGZyb20gJ3BhdGgnO1xuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cblxuXG5leHBvcnQgZGVmYXVsdCBkZWZpbmVDb25maWcoe1xuICBwbHVnaW5zOiBbdHNjb25maWdQYXRocygpLCBzdmVsdGUoKSwgdml0ZVN0YXRpY0NvcHkoe1xuICAgIHRhcmdldHM6IFtcbiAgICAgIHtcbiAgICAgICAgc3JjOiBub3JtYWxpemVQYXRoKHJlc29sdmUoX19kaXJuYW1lLCAnLi9zcGxhc2hzY3JlZW4vaW5kZXguaHRtbCcpKSxcbiAgICAgICAgZGVzdDogICdzcGxhc2hzY3JlZW4nLFxuICAgICAgfSxcbiAgICAgIHtcbiAgICAgICAgc3JjOiBub3JtYWxpemVQYXRoKHJlc29sdmUoX19kaXJuYW1lLCAnLi9zcGxhc2hzY3JlZW4vc3R5bGUuY3NzJykpLFxuICAgICAgICBkZXN0OiAgJ3NwbGFzaHNjcmVlbicsXG4gICAgICB9LFxuICAgIF1cbiAgfSldLFxuICBzZXJ2ZXI6IHtcbiAgICBwb3J0OiAzMDAwLFxuICB9LFxuICBvcHRpbWl6ZURlcHM6IHsgZXhjbHVkZTogW1wic3ZlbHRlLW5hdmlnYXRvclwiXSB9LFxuICBjc3M6e1xuICAgIHBvc3Rjc3NcbiAgfVxufSkiXSwKICAibWFwcGluZ3MiOiAiOzs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7QUFBQTtBQUFBO0FBQ0EsV0FBTyxVQUFVO0FBQUEsTUFDZixTQUFTLENBQUMsZ0JBQWdCLDJCQUEyQjtBQUFBLE1BQ3JELFNBQVM7QUFBQTtBQUFBLFFBRVAsVUFBVSxDQUFDLEdBQUcsTUFBTSxLQUFLLEVBQUUsUUFBUSxHQUFJLENBQUMsRUFBRSxLQUFLLEVBQUUsRUFBRSxJQUFJLENBQUMsR0FBRyxNQUFNLFdBQVcsSUFBSSxDQUFDLEVBQUUsQ0FBQztBQUFBLE1BQ3RGO0FBQUEsTUFDQSxPQUFPO0FBQUEsUUFDTCxRQUFRO0FBQUEsVUFDTixZQUFZO0FBQUEsWUFDVixVQUFVLENBQUMsWUFBWSxZQUFZO0FBQUEsWUFDbkMsYUFBYSxDQUFDLGVBQWdCLFNBQVM7QUFBQSxVQUN6QztBQUFBLFFBQ0Y7QUFBQSxNQUNGO0FBQUEsTUFDQSxTQUFTLENBQUM7QUFBQSxJQUNaO0FBQUE7QUFBQTs7O0FDZEEsNkJBQTJCO0FBRjRRLE9BQU8sY0FBYztBQUM1VCxPQUFPLGtCQUFrQjtBQUd6QixJQUFPLHlCQUFRO0FBQUEsRUFDYixTQUFTLENBQUMsU0FBUyx1QkFBQUEsT0FBYyxHQUFHLFlBQVk7QUFDbEQ7OztBQ0xBLFNBQVMsY0FBYztBQUN2QixPQUFPLG1CQUFtQjtBQUMxQixTQUFTLG9CQUFvQjtBQUM3QixTQUFTLHNCQUFzQjtBQUMvQixTQUFTLHFCQUFxQjtBQUM5QixTQUFTLGVBQWU7QUFOeEIsSUFBTSxtQ0FBbUM7QUFVekMsSUFBTyxzQkFBUSxhQUFhO0FBQUEsRUFDMUIsU0FBUyxDQUFDLGNBQWMsR0FBRyxPQUFPLEdBQUcsZUFBZTtBQUFBLElBQ2xELFNBQVM7QUFBQSxNQUNQO0FBQUEsUUFDRSxLQUFLLGNBQWMsUUFBUSxrQ0FBVywyQkFBMkIsQ0FBQztBQUFBLFFBQ2xFLE1BQU87QUFBQSxNQUNUO0FBQUEsTUFDQTtBQUFBLFFBQ0UsS0FBSyxjQUFjLFFBQVEsa0NBQVcsMEJBQTBCLENBQUM7QUFBQSxRQUNqRSxNQUFPO0FBQUEsTUFDVDtBQUFBLElBQ0Y7QUFBQSxFQUNGLENBQUMsQ0FBQztBQUFBLEVBQ0YsUUFBUTtBQUFBLElBQ04sTUFBTTtBQUFBLEVBQ1I7QUFBQSxFQUNBLGNBQWMsRUFBRSxTQUFTLENBQUMsa0JBQWtCLEVBQUU7QUFBQSxFQUM5QyxLQUFJO0FBQUEsSUFDRjtBQUFBLEVBQ0Y7QUFDRixDQUFDOyIsCiAgIm5hbWVzIjogWyJ0YWlsd2luZENvbmZpZyJdCn0K
