<script lang="ts">
    import {
        parse_html_to_markdown,
        type ParseOutput,
        convert_parsed_markdown_to_html,
    } from "../../crates/app/pkg/app";
    import { onMount } from "svelte";

    let { goBack, skipManual = false } = $props<{
        goBack: () => void;
        skipManual: boolean;
    }>();

    let content = $state<string>("");
    let mode = $state<"dialog" | "reader">("dialog");

    let fileInput: HTMLInputElement;

    const openReader = () => {
        fileInput?.click();
    };

    const handleFileChange = async (event: Event): Promise<void> => {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) {
            goBack();
            return;
        }

        const text: string = await file.text();
        const mdData: ParseOutput = parse_html_to_markdown(text);

        content = convert_parsed_markdown_to_html(mdData.markdown);
        mode = "reader";
    };

    onMount(() => {
        if (skipManual) {
            openReader();
        }
    });
</script>

{#if mode === "dialog" && !skipManual}
    <div class="centerWrapper">
        <div class="centered">
            <header>How to load a document</header>
            <p>...</p>

            <div class="buttons">
                <button class="btn" onclick={openReader}>Open</button>
                <button class="btn" onclick={goBack}>Back</button>
            </div>
        </div>
    </div>
{:else if mode === "reader"}
    <div>
        <button class="btn" onclick={goBack}>Back</button>
    </div>

    <!-- Insert raw HTML -->
    <div class="reader">{@html content}</div>
{/if}

<input
    bind:this={fileInput}
    type="file"
    accept=".html"
    hidden
    onchange={handleFileChange}
    oncancel={handleFileChange}
/>

<style>
    .reader {
        max-width: 800px;
        margin: 20px auto;
        line-height: 1.3rem;
    }
</style>
