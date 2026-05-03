<script lang="ts">
    import { parse_md } from "@wasm/app";
    import { updateReadPath } from "@/router";
    import { Errors } from "@/errorHandler";
    import { getMarkdown, getHeadings, type Heading } from "@/db";
    import { onMount } from "svelte";

    const { goBack, readHash, displayError } = $props<{
        goBack: () => void;
        readHash: string;
        displayError?: string;
    }>();

    let content = $state<string>("");
    let error = $state<string | undefined>();
    let headings = $state<Heading[]>();
    let showPanel = $state(false);

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

        const headingsRecord = await getHeadings(readHash);
        headings = headingsRecord?.headings;

        const { html, heading_levels, heading_lines, heading_contents } =
            parse_md(data.value);

        content = html;
    });
</script>

<button class="btn back" onclick={goBack}>Back</button>
<button class="btn toggle" onclick={() => (showPanel = !showPanel)}> × </button>

<div class="panel" class:open={showPanel}>
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
    <b>{record.content}</b>
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
        top: 50px;
        left: 0;
        height: calc(100% - 70px);
        max-width: 100px;

        display: flex;
        flex-direction: column;
        gap: 10px;
        padding: 10px;

        background: var(--bg);

        transform: translateX(-100%);
        opacity: 0;
        pointer-events: none;

        transition:
            transform 0.25s ease,
            opacity 0.2s ease;
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
        gap: 10px;
    }

    .errorCont {
        margin: 10px;
        text-align: center;
        color: #ff4b53;
    }

    b {
        word-break: break-word;
    }

    @media (min-width: 769px) {
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
