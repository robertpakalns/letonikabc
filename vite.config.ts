import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [svelte()],
  build: {
    target: "esnext",
    sourcemap: false,
    minify: "terser",
    terserOptions: {
      compress: {
        passes: 7,
        inline: 3,
        booleans_as_integers: true,
        reduce_funcs: true,
        reduce_vars: true,
      },
      format: {
        comments: false,
      },
      mangle: true,
    },
    polyfillModulePreload: false,
  },
});
