<script lang="ts">
    import {
        parse_html_to_markdown,
        type ParseOutput,
        convert_parsed_markdown_to_html,
    } from "../crates/app/pkg/app";

    let content: string = "";

    const handleFileChange = async (event: Event): Promise<void> => {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) return;

        const text: string = await file.text();
        const mdData: ParseOutput = parse_html_to_markdown(text);

        content = convert_parsed_markdown_to_html(mdData.markdown);
    };
</script>

<input type="file" accept=".html" on:change={handleFileChange} />

<!-- Insert raw HTML -->
<div>{@html content}</div>
