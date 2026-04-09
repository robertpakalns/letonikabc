const BASE = "/letonikabc/";

const updateHistory = (path: string): void => {
  history.pushState({}, "", `${BASE}${path}`);
};

export type Route =
  | { route: "menu" }
  | { route: "new" }
  | { route: "load" }
  | { route: "reader"; hash: string };

export const parseRoute = (): Route => {
  const path = window.location.pathname;

  if (!path.startsWith(BASE)) return { route: "menu" };

  const subPath = path.slice(BASE.length);
  const parts = subPath.split("/").filter(Boolean);

  if (parts.length === 0) return { route: "menu" };
  if (parts[0] === "new") return { route: "new" };
  if (parts[0] === "load") return { route: "load" };

  if (parts[0] === "read" && parts[1]) {
    return { route: "reader", hash: parts[1] };
  }

  return { route: "menu" };
};

export const updateReadPath = (hash: string): void => {
  updateHistory(`read/${hash}`);
};

export const navigateToNew = (): void => {
  updateHistory("new");
};

export const navigateToMain = (): void => {
  updateHistory("");
};

export const navigateToLoad = (): void => {
  updateHistory("load");
};
