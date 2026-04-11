<script lang="ts">
    import { getSettings, updateFont } from "@/storage";
    import { applyFont, type Font } from "@/fonts";
    import { onMount } from "svelte";

    let currentFont: Font = "sans-serif";

    const changeFont = (type: Font): void => {
        currentFont = type;
        applyFont(type);
        updateFont(type);
    };

    onMount(() => {
        const settings = getSettings();
        currentFont = settings.font;
        applyFont(currentFont);
    });
</script>

<div>
    <header>Settings</header>

    <div class="buttons">
        <button class="btn" onclick={() => changeFont("sans-serif")}>
            Sans-serif
        </button>
        <button class="btn" onclick={() => changeFont("serif")}>Serif</button>
        <button class="btn" onclick={() => changeFont("monospace")}>
            Monospace
        </button>
    </div>
</div>
