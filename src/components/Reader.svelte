<script lang="ts">
    import { convert_parsed_markdown_to_html } from "../../pkg/app";
    import { getRecord } from "../db";
    import { onMount } from "svelte";

    let { goBack, readHash, displayError } = $props<{
        goBack: () => void;
        readHash: string;
        displayError: string | undefined;
    }>();

    let content = $state<string>("");

    onMount(async () => {
        const data = await getRecord(readHash);
        if (!data) return;

        const html = convert_parsed_markdown_to_html(data.value);

        content = html;
    });
</script>

<div class="btn-panel">
    <button class="btn" onclick={goBack}>Back</button>
</div>

{#if displayError !== undefined}
    <div class="errorCont">{displayError}</div>
{/if}

<!-- Insert raw HTML -->
<div class="reader">{@html content}</div>

<style>
    .errorCont {
        margin: 10px;
        width: 100%;
        text-align: center;
        color: #ff4b53;
    }
</style>
