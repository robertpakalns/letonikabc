export type Theme = "system" | "dark" | "light";

export const isValidTheme = (theme: Theme): boolean => {
  return theme === "system" || theme === "dark" || theme === "light";
};

export const applyTheme = (theme: Theme) => {
  const root = document.documentElement;

  if (theme === "system") {
    root.removeAttribute("data-theme"); // Fallback to media query
  } else {
    root.setAttribute("data-theme", theme);
  }
};
