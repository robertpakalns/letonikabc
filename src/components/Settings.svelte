<script lang="ts">
    type Font = "sans-serif" | "serif" | "monospace";

    const fonts: Record<Font, string> = {
        "sans-serif": "Google Sans, sans-serif",
        serif: "serif",
        monospace: "monospace",
    };

    const changeFont = (type: Font): void => {
        document.documentElement.style.setProperty("--font", fonts[type]);
    };
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
