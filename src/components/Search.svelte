<script lang="ts">
    import { find_substr } from "@wasm/app";
    import { getMarkdown } from "@/db";
    import { onMount } from "svelte";

    const { readHash } = $props<{ readHash: string }>();

    type Match = { start: number; end: number };

    const CONTEXT = 50;
    const CONTEXT_LINES = 2;

    let result = $state("");

    let query = $state("");
    let caseSensitive = $state(true);
    let diacriticSensitive = $state(true);

    const escapeHtml = (str: string): string =>
        str
            .replaceAll("&", "&amp;")
            .replaceAll("<", "&lt;")
            .replaceAll(">", "&gt;");

    const buildLineIndex = (str: string): number[] => {
        const map: number[] = [];
        let line = 0;

        for (let i = 0; i < str.length; i++) {
            map[i] = line;
            if (str[i] === "\n") line++;
        }

        return map;
    };

    const highlight = (line: string, matches: Match[]): string => {
        if (!matches.length) return escapeHtml(line);

        let out = "";
        let cursor = 0;

        for (const m of matches) {
            out += escapeHtml(line.slice(cursor, m.start));
            out += `<mark>${escapeHtml(line.slice(m.start, m.end))}</mark>`;
            cursor = m.end;
        }

        out += escapeHtml(line.slice(cursor));
        return out;
    };

    const sliceWithContext = (
        line: string,
        matches: Match[],
    ): { text: string; matches: Match[] }[] => {
        return matches.map((m) => {
            const rawStart = Math.max(0, m.start - CONTEXT);
            const rawEnd = Math.min(line.length, m.end + CONTEXT);

            const hasPrefix = rawStart > 0;
            const hasSuffix = rawEnd < line.length;

            const prefix = hasPrefix ? "…" : "";
            const suffix = hasSuffix ? "…" : "";

            return {
                text: prefix + line.slice(rawStart, rawEnd) + suffix,
                matches: [
                    {
                        start: m.start - rawStart + prefix.length,
                        end: m.end - rawStart + prefix.length,
                    },
                ],
            };
        });
    };

    const trimLine = (line: string, maxLen = 120): string => {
        if (line.length <= maxLen) return escapeHtml(line);
        return escapeHtml(line.slice(0, maxLen)) + "…";
    };

    let matches = $derived(
        query
            ? find_substr(result, query, !diacriticSensitive, caseSensitive)
            : [],
    );

    let lines = $derived(result.split("\n"));
    let lineIndex = $derived(buildLineIndex(result));
    let matchCount = $derived(matches.length);

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

<div class="controls">
    <input
        class="textInput"
        type="text"
        placeholder="Search..."
        bind:value={query}
    />

    <span class="count">{matchCount}</span>

    <label>
        <input type="checkbox" bind:checked={caseSensitive} />
        Case-sensitive
    </label>

    <label>
        <input type="checkbox" bind:checked={diacriticSensitive} />
        Diacritic-sensitive
    </label>
</div>

<div class="results">
    {#if result && grouped.length}
        {#each grouped as g}
            {@const start = Math.max(0, g.line - CONTEXT_LINES)}
            {@const end = Math.min(lines.length - 1, g.line + CONTEXT_LINES)}

            <div class="block">
                {#each lines.slice(start, end + 1) as lineText, i}
                    {@const actualLine = start + i}

                    <div class="line" class:active={actualLine === g.line}>
                        {#if actualLine === g.line}
                            {#each sliceWithContext(lineText, g.matches) as seg}
                                <div class="segment">
                                    {@html highlight(seg.text, seg.matches)}
                                </div>
                            {/each}
                        {:else}
                            <div>{@html trimLine(lineText)}</div>
                        {/if}
                    </div>
                {/each}
            </div>
        {/each}
    {:else}
        <div class="empty">Nothing found</div>
    {/if}
</div>

<style>
    .controls {
        display: flex;
        gap: 10px;
        align-items: center;
        margin-bottom: 10px;
        font-family: monospace;

        input[type="text"] {
            flex: 1;
            padding: 4px 6px;
            font-family: monospace;
        }

        label {
            font-size: 12px;
            opacity: 0.8;
            display: flex;
            align-items: center;
            gap: 4px;
        }
    }

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

    .segment {
        margin-bottom: 2px;
    }

    .empty {
        opacity: 0.6;
    }

    .textInput {
        background: transparent;
        outline: none;
        border: none;
        border-bottom: 1px solid gray;
    }
</style>
