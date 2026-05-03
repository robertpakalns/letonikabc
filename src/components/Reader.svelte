<script lang="ts">
    import { convert_parsed_markdown_to_html } from "@wasm/app";
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

    onMount(async () => {
        updateReadPath(readHash);

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

        const html = convert_parsed_markdown_to_html(data.value);
        content = html;
    });
</script>

<div class="btn-panel">
    <button class="btn" onclick={goBack}>Back</button>
</div>

{#if error}
    <div class="errorCont">{error}</div>
{/if}

<div class="headings">
    {#each headings as record}
        {@render h(record)}
    {/each}
</div>

{#snippet h(record: Heading)}
    <b>{record.content}</b>
{/snippet}

<!-- Insert raw HTML -->
<div class="reader">{@html content}</div>

<style>
    .errorCont {
        margin: 10px;
        text-align: center;
        color: #ff4b53;
    }

    .headings {
        position: absolute;
        padding: 10px;
        top: 50%;
        left: 0;
        transform: translateY(-50%);
        height: 50%;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 10px;

        background: var(--bg);
    }
</style>
