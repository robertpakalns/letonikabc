import { getMarkdown, getMetadata } from "./db";

export const formatBytes = (bytes: number): string => {
  const units = ["B", "KB", "MB", "GB"];
  let i = 0;

  while (bytes >= 1024 && i < units.length - 1) {
    bytes /= 1024;
    i++;
  }

  return `${bytes.toFixed(2)} ${units[i]}`;
};

export const exportMarkdown = async (hash: string): Promise<void> => {
  const md = await getMarkdown(hash);
  if (!md) return;

  const meta = await getMetadata(hash);
  if (!meta) return;

  const blob = new Blob([md.value], { type: "text/markdown;charset=utf-8" });

  const url = URL.createObjectURL(blob);

  const a = document.createElement("a");
  a.href = url;
  a.download = `letonikabc_${meta.author || "unknown"}_${meta.title || "unknown"}_${meta.created_at}.md`;

  document.body.appendChild(a);
  a.click();

  document.body.removeChild(a);
  URL.revokeObjectURL(url);
};
