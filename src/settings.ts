// Types
export type Font = "sans-serif" | "serif" | "monospace";
export type Theme = "system" | "dark" | "light";
export type BGColor = "default" | string;

// Validation
const isValidFont = (font: Font): boolean =>
  font === "sans-serif" || font === "serif" || font === "monospace";
const isValidTheme = (theme: Theme): boolean =>
  theme === "system" || theme === "dark" || theme === "light";
const isValidBGColor = (color: BGColor): boolean => {
  const s = new Option().style;
  s.color = color;
  return s.color !== "";
};

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
export const applyBGColor = (c: BGColor) => {
  let color =
    c === "default"
      ? getComputedStyle(document.documentElement)
          .getPropertyValue("--bg")
          .trim()
      : c;

  document.documentElement.style.setProperty("--bg", color);
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
export const updateBGColor = (color: BGColor): void => {
  const settings = getSettings();
  settings.bgColor = color;
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));

  applyBGColor(color);
};

// Settings
const STORAGE_KEY = "settings";

type Settings = {
  font: Font;
  theme: Theme;
  bgColor: BGColor;
};

const defaultSettings = (): Settings => ({
  font: "sans-serif",
  theme: "system",
  bgColor: "default",
});

export const getSettings = (): Settings => {
  const raw = localStorage.getItem(STORAGE_KEY);
  if (!raw) return defaultSettings();

  try {
    const parsed = JSON.parse(raw);

    if (
      parsed &&
      isValidFont(parsed.font) &&
      isValidTheme(parsed.theme) &&
      isValidBGColor(parsed.bgColor)
    ) {
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
  applyBGColor(settings.bgColor);
};
