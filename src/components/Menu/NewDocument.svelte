<script lang="ts">
    import { parse_html_to_markdown } from "../../../crates/app/pkg/app";
    import { addRecord } from "../../db";
    import { onMount } from "svelte";

    const {
        goBack,
        skipManual = false,
        goRead,
    } = $props<{
        goBack: () => void;
        skipManual: boolean;
        goRead: (idx: number) => void;
    }>();

    let fileInput: HTMLInputElement;

    const requestFile = () => {
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
        const mdData = parse_html_to_markdown(text);
        const { markdown } = mdData;

        const idx = await addRecord(markdown);

        goRead(idx);
    };

    onMount(() => {
        if (skipManual) {
            requestFile();
        }
    });
</script>

<div class="centerWrapper">
    <div class="centered">
        <header>How to load a document</header>
        <ol>
            <li>
                Get a raw document from <a
                    href="https://www.letonika.lv/"
                    target="_blank"
                >
                    letonika.lv
                </a>
            </li>
            <li>Save it on local machine</li>
            <li>Click "Open"</li>
            <li>Choose the document</li>
        </ol>

        <div class="buttons">
            <button class="btn" onclick={requestFile}>Open</button>
            <button class="btn" onclick={goBack}>Back</button>
        </div>
    </div>
</div>

<input
    bind:this={fileInput}
    type="file"
    accept=".html"
    hidden
    onchange={handleFileChange}
    oncancel={handleFileChange}
/>

<style>
    ol {
        padding-left: 2rem;
        /*font-style: italic;*/
    }
</style>
