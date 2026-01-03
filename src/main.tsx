import { createRoot } from "react-dom/client";
import init from "../crates/app/pkg/app";
import App from "./App";

const bootstrap = async (): Promise<void> => {
  await init();
  createRoot(document.getElementById("app")!).render(<App />);
};

bootstrap();
