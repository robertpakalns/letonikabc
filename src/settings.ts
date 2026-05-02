// Types
export type Font = "sans-serif" | "serif" | "monospace";
export type Theme = "system" | "dark" | "light";
type Settings = {
  font: Font;
  theme: Theme;
};

// Validation
const isValidFont = (font: Font): boolean =>
  font === "sans-serif" || font === "serif" || font === "monospace";
const isValidTheme = (theme: Theme): boolean =>
  theme === "system" || theme === "dark" || theme === "light";

// Apply
const fonts: Record<Font, string> = {
  "sans-serif": "'Figtree', sans-serif",
  serif: "'Source Serif 4', serif",
  monospace: "'Roboto Mono', monospace",
};
export const applyFont = (type: Font) => {
  document.documentElement.style.setProperty("--font", fonts[type]);
};
export const applyTheme = (theme: Theme) => {
  const root = document.documentElement;

  if (theme === "system") {
    root.removeAttribute("data-theme"); // Fallback to media query
  } else {
    root.setAttribute("data-theme", theme);
  }
};

// Update localStorage
export const updateFont = (font: Font): void => {
  const settings = getSettings();
  settings.font = font;
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));

  applyFont(font);
};
export const updateTheme = (theme: Theme): void => {
  const settings = getSettings();
  settings.theme = theme;
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));

  applyTheme(theme);
};

// Settings
const STORAGE_KEY = "settings";

const defaultSettings = (): Settings => ({
  font: "sans-serif",
  theme: "system",
});

export const getSettings = (): Settings => {
  const raw = localStorage.getItem(STORAGE_KEY);
  if (!raw) return defaultSettings();

  try {
    const parsed = JSON.parse(raw);

    if (parsed && isValidFont(parsed.font) && isValidTheme(parsed.theme)) {
      return parsed;
    }

    return defaultSettings();
  } catch {
    return defaultSettings();
  }
};

export const applySettings = (): void => {
  const settings = getSettings();
  applyFont(settings.font);
  applyTheme(settings.theme);
};
