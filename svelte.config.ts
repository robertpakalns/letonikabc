import { svelte } from "@sveltejs/vite-plugin-svelte";
import type { UserConfig } from "vite";

const config: UserConfig = {
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
