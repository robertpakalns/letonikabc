import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";
import { resolve } from "path";

export default defineConfig({
  resolve: {
    alias: {
      "@": resolve("src"),
      "@wasm": resolve("pkg"),
    },
  },
  base: "/letonikabc/",
  plugins: [svelte()],
  build: {
    target: "esnext",
    sourcemap: false,
    minify: true,
    modulePreload: {
      polyfill: false,
    },
    rolldownOptions: {
      output: {
        manualChunks(id) {
          if (id.includes("/pkg/")) return "core";
          if (id.includes("node_modules")) return "vendor";
        },
      },
    },
  },
});
