<script lang="ts">
    import { getMarkdown, getHeadings, addHeadings } from "@/db";
    import { onDestroy, onMount } from "svelte";
    import { updateReadPath } from "@/router";
    import { Errors } from "@/errorHandler";
    import { parse_md } from "@wasm/app";
    import type { Heading } from "@/db";

    const { goBack, readHash, displayError } = $props<{
        goBack: () => void;
        readHash: string;
        displayError?: string;
    }>();

    let content = $state<string>("");
    let error = $state<string | undefined>();
    let headings = $state<Heading[]>();
    let showPanel = $state(false);

    const scrollToId = (id: string): void => {
        document.getElementById(id)?.scrollIntoView({ behavior: "smooth" });
    };

    let panelEl: HTMLDivElement | null = null;
    let toggleBtn: HTMLButtonElement | null = null;

    const handleClick = (e: MouseEvent) => {
        if (!showPanel) return;

        const target = e.target as Node;

        if (
            panelEl &&
            !panelEl.contains(target) &&
            toggleBtn &&
            !toggleBtn.contains(target)
        ) {
            showPanel = false;
        }
    };

    onMount(async () => {
        updateReadPath(readHash);

        if (window.innerWidth > 768) {
            showPanel = true;
        }

        if (displayError) {
            error = displayError;
        }

        const data = await getMarkdown(readHash);
        if (!data) {
            error = Errors.NotFound;
            return;
        }

        const { html, heading_levels, heading_lines, heading_contents } =
            parse_md(data.value);

        const headingsArr: Heading[] = Array.from(heading_lines).map(
            (line, i) => ({
                line,
                content: heading_contents[i],
                level: heading_levels[i],
            }),
        );

        const headingsRecord = await getHeadings(readHash);
        headings = headingsRecord?.headings || headingsArr;

        await addHeadings({ hash: readHash, headings: headingsArr });

        content = html;

        document.addEventListener("click", handleClick);
    });

    onDestroy(() => {
        document.removeEventListener("click", handleClick);
    });
</script>

<button class="btn back" onclick={goBack}>Back</button>
<button
    class="btn toggle"
    bind:this={toggleBtn}
    onclick={() => (showPanel = !showPanel)}
>
    ×
</button>

<div class="panel" bind:this={panelEl} class:open={showPanel}>
    <div class="headings">
        {#each headings as record}
            {@render h(record)}
        {/each}
    </div>
</div>

{#if error}
    <div class="errorCont">{error}</div>
{/if}

{#snippet h(record: Heading)}
    <button
        style={`padding-left: ${record.level * 4}px; font-size: ${1 - (record.level - 1) * 0.12}rem;`}
        onclick={() => scrollToId(record.line.toString())}
    >
        {record.content}
    </button>
{/snippet}

<!-- Insert raw HTML -->
<div class="reader">{@html content}</div>

<style>
    .toggle {
        position: absolute;
        top: 10px;
        left: 80px;
        z-index: 10;
    }

    .back {
        position: absolute;
        top: 10px;
        left: 10px;
        z-index: 10;
    }

    .panel {
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        height: calc(100% - 50px);
        max-width: 100px;

        padding-top: 50px;

        display: flex;
        flex-direction: column;

        background: color-mix(in srgb, var(--bg) 80%, transparent);

        transform: translateX(-100%);
        opacity: 0;
        pointer-events: none;

        transition: 0.3s ease;
    }

    .panel.open {
        transform: translateX(0);
        opacity: 1;
        pointer-events: auto;
    }

    .headings {
        padding: 10px;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 7px;
    }

    .errorCont {
        margin: 10px;
        text-align: center;
        color: #ff4b53;
    }

    button:not(.btn) {
        padding: 0.5rem;
        font-weight: 600;
        font-size: 1rem;
        word-break: break-word;
        text-align: left;
        background: transparent;
        outline: none;
        border: none;
        cursor: pointer;
        transition: 0.3s ease;
        border-radius: 6px;

        &:hover {
            transform: scale(1.1);
            background: gray;
        }
    }

    @media (min-width: 900px) {
        .panel {
            transform: translateX(0);
            opacity: 1;
            pointer-events: auto;
        }

        .toggle {
            display: none;
        }
    }
</style>
