import { type Theme, isValidTheme } from "@/themes";
import { type Font, isValidFont } from "@/fonts";

const STORAGE_KEY = "settings";

type Settings = {
  font: Font;
  theme: Theme;
};

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

export const updateFont = (font: Font): void => {
  const settings = getSettings();
  settings.font = font;
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
};

export const updateTheme = (theme: Theme): void => {
  const settings = getSettings();
  settings.theme = theme;
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
};
