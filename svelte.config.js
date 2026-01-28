import { svelte } from "@sveltejs/vite-plugin-svelte";

/** @type {import('vite').UserConfig} */
const config = {
  plugins: [
    svelte({
      compilerOptions: {
        dev: false,
      },
      emitCss: true,
    }),
  ],
};

export default config;
