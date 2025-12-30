import init from "../crates/parser/pkg/parser";
import { createRoot } from "react-dom/client";
import App from "./App";

const bootstrap = async (): Promise<void> => {
  await init();
  createRoot(document.getElementById("app")!).render(<App />);
};

bootstrap();
