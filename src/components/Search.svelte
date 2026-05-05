<script lang="ts">
    import { find_substr } from "@wasm/app";
    import { getMarkdown } from "@/db";
    import { onMount } from "svelte";

    const { readHash } = $props<{ readHash: string }>();

    type Match = { start: number; end: number };

    const buildLineIndex = (str: string): number[] => {
        const map: number[] = [];

        let line = 0;
        for (let i = 0; i < str.length; i++) {
            map[i] = line;
            if (str[i] === "\n") line++;
        }

        return map;
    };

    const escapeHtml = (str: string): string => {
        return str
            .replaceAll("&", "&amp;")
            .replaceAll("<", "&lt;")
            .replaceAll(">", "&gt;");
    };

    const highlight = (line: string, matches: Match[]): string => {
        if (!matches.length) return escapeHtml(line);

        let out = "";
        let cursor = 0;

        for (const m of matches) {
            const start = m.start;
            const end = m.end;

            out += escapeHtml(line.slice(cursor, start));
            out += `<mark>${escapeHtml(line.slice(start, end))}</mark>`;

            cursor = end;
        }

        out += escapeHtml(line.slice(cursor));
        return out;
    };

    let result = $state<string>("");
    const query = "mind";
    let matches = $derived(find_substr(result, query, true, false));
    let lines = $derived(result.split("\n"));
    let lineIndex = $derived(buildLineIndex(result));

    let grouped = $derived(
        (() => {
            const map = new Map<number, Match[]>();

            const len = query.length;

            for (const globalStart of matches) {
                const line = lineIndex[globalStart] ?? 0;

                let lineStart = globalStart;
                while (lineStart > 0 && lineIndex[lineStart] === line) {
                    lineStart--;
                }
                if (lineIndex[lineStart] !== line) lineStart++;

                const localStart = globalStart - lineStart;
                const localEnd = localStart + len;

                const arr = map.get(line) ?? [];
                arr.push({ start: localStart, end: localEnd });
                map.set(line, arr);
            }

            return Array.from(map.entries()).map(([line, matches]) => ({
                line,
                matches: matches.sort((a, b) => a.start - b.start),
            }));
        })(),
    );

    onMount(async () => {
        const data = await getMarkdown(readHash);
        result = data?.value ?? "";
    });
</script>

<div class="results">
    {#if result}
        {#each grouped as g}
            {@const start = Math.max(0, g.line - 2)}
            {@const end = Math.min(lines.length - 1, g.line + 2)}

            <div class="block">
                {#each lines.slice(start, end + 1) as lineText, i}
                    {@const actualLine = start + i}

                    <div class="line" class:active={actualLine === g.line}>
                        {@html highlight(
                            lineText,
                            actualLine === g.line ? g.matches : [],
                        )}
                    </div>
                {/each}
            </div>
        {/each}
    {:else}
        <div class="empty">Nothing found</div>
    {/if}
</div>

<style>
    .results {
        font-family: monospace;
        padding: 10px;

        max-width: 800px;

        max-height: 80vh;
        overflow-y: auto;
        overflow-x: hidden;
    }

    .block {
        margin-bottom: 14px;
        border-left: 2px solid #333;
        padding-left: 10px;
    }

    .line {
        white-space: pre-wrap;
        word-break: break-word;

        padding: 2px 0;
        opacity: 0.7;
    }

    .line.active {
        opacity: 1;
        background: rgba(255, 255, 0, 0.08);
    }

    .empty {
        opacity: 0.6;
    }
</style>
