import init from "../crates/app/pkg/app";
import { mount } from "svelte";
import App from "./App.svelte";

await init();

mount(App, { target: document.getElementById("app")! });
