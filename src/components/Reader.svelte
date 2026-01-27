<script lang="ts">
    import { convert_parsed_markdown_to_html } from "../../crates/app/pkg/app";
    import { getRecord } from "../db";
    import { onMount } from "svelte";

    let { goBack, readId } = $props<{
        goBack: () => void;
        readId: number;
    }>();

    let content = $state<string>("");

    onMount(async () => {
        const data = await getRecord(readId);
        if (!data) return;

        const html = convert_parsed_markdown_to_html(data.value);

        content = html;
    });
</script>

<div class="btn-panel">
    <button class="btn" onclick={goBack}>Back</button>
</div>

<!-- Insert raw HTML -->
<div class="reader">{@html content}</div>
