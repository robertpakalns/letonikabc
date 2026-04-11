export type Font = "sans-serif" | "serif" | "monospace";

export const isValidFont = (font: Font): boolean => {
  return font === "sans-serif" || font === "serif" || font === "monospace";
};

const fonts: Record<Font, string> = {
  "sans-serif": "'Google Sans', sans-serif",
  serif: "'Source Serif 4', serif",
  monospace: "'Roboto Mono', monospace",
};

export const applyFont = (type: Font) => {
  document.documentElement.style.setProperty("--font", fonts[type]);
};
