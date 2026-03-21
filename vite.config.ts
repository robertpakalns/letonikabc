import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";

export default defineConfig({
  base: "/letonikabc/",
  plugins: [svelte()],
  build: {
    target: "esnext",
    sourcemap: false,
    minify: true,
    modulePreload: {
      polyfill: false,
    },
  },
});
