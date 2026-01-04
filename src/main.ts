import { mount } from "svelte";
import init from "../crates/app/pkg/app";
import App from "./App.svelte";

await init();

mount(App, {
  target: document.getElementById("app")!,
});
