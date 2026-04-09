<script lang="ts">
    import { convert_parsed_markdown_to_html } from "@wasm/app";
    import { updateReadPath } from "@/router";
    import { Errors } from "@/errorHandler";
    import { getMarkdown } from "@/db";
    import { onMount } from "svelte";

    const { goBack, readHash, displayError } = $props<{
        goBack: () => void;
        readHash: string;
        displayError?: string;
    }>();

    let content = $state<string>("");
    let error = $state<string | undefined>();

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

<!-- Insert raw HTML -->
<div class="reader">{@html content}</div>

<style>
    .errorCont {
        margin: 10px;
        text-align: center;
        color: #ff4b53;
    }
</style>
