const BASE = "/letonikabc/";

export const extractReadHash = (): string | null => {
  const path = window.location.pathname;
  if (!path.startsWith(BASE)) return null;

  const subPath = path.slice(BASE.length);
  const parts = subPath.split("/");

  if (parts.length !== 2 || parts[0] !== "read") return null;

  return parts[1];
};

export const updateReadPath = (hash: string): void => {
  history.pushState({}, "", `${BASE}read/${hash}`);
};
