<script lang="ts">
    import { parse_html_to_markdown } from "../../../pkg/app";

    import { addMarkdown, MDRecord } from "../../db";
    import { onMount } from "svelte";

    const {
        goBack,
        skipManual = false,
        goRead,
    } = $props<{
        goBack: () => void;
        skipManual: boolean;
        goRead: (readHash: string, error?: string) => void;
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
        const { markdown, hash, heading_lines } = mdData;

        const record: MDRecord = { hash, value: markdown };

        console.log(heading_lines);

        const { hash: readHash, error } = await addMarkdown(record);

        goRead(readHash, error);
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
    }
</style>
