import { mount } from "svelte";
import App from "./App.svelte";
import init from "@wasm/app";
import "./style.css";

await init();

mount(App, { target: document.getElementById("app")! });
