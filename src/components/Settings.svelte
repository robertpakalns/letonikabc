<script lang="ts">
    import { updateFont, updateTheme } from "@/storage";
    import { applyTheme, type Theme } from "@/themes";
    import { applyFont, type Font } from "@/fonts";

    const changeFont = (type: Font): void => {
        applyFont(type);
        updateFont(type);
    };

    const changeTheme = (type: Theme): void => {
        applyTheme(type);
        updateTheme(type);
    };
</script>

<div>
    <header>Settings</header>

    <div class="section">
        <h3>Font</h3>
        <div class="buttons">
            <button class="btn" onclick={() => changeFont("sans-serif")}>
                Sans-serif
            </button>
            <button class="btn" onclick={() => changeFont("serif")}>
                Serif
            </button>
            <button class="btn" onclick={() => changeFont("monospace")}>
                Monospace
            </button>
        </div>
    </div>

    <div class="section">
        <h3>Theme</h3>
        <div class="buttons">
            <button class="btn" onclick={() => changeTheme("system")}>
                System
            </button>
            <button class="btn" onclick={() => changeTheme("dark")}>
                Dark
            </button>
            <button class="btn" onclick={() => changeTheme("light")}>
                Light
            </button>
        </div>
    </div>
</div>

<style>
    h3 {
        text-align: center;
    }

    .buttons {
        margin: 5px 0 0 0;
    }

    .section {
        margin: 10px 0;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>
