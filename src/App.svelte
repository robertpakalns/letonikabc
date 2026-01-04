<script lang="ts">
    import {
        parse_html_to_markdown,
        type ParseOutput,
    } from "../crates/app/pkg/app";

    let data: ParseOutput | null = null;

    const handleFileChange = async (event: Event): Promise<void> => {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) return;

        const text = await file.text();
        data = parse_html_to_markdown(text);
    };
</script>

<input type="file" accept=".html" on:change={handleFileChange} />

{#if data}
    <div>
        <div>{data.markdown}</div>
        <div>{Array.from(data.header_lines).join(", ")}</div>
    </div>
{/if}
