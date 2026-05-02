<script lang="ts">
    import {
        getSettings,
        updateFont,
        updateTheme,
        updateBGColor,
        applyBGColor,
        resetSettings,
        defaultSettings,
    } from "@/settings";
    import type { Theme, Font, BGColor } from "@/settings";

    import { onMount } from "svelte";

    let font: Font;
    let theme: Theme;
    let bgColor: BGColor;

    const changeFont = (type: Font): void => {
        font = type;
        updateFont(type);
    };
    const changeTheme = (type: Theme): void => {
        theme = type;
        updateTheme(type);
    };

    const changeBGColor = (c: BGColor): void => {
        updateBGColor(c);
        bgColor = chooseColor(c);
    };
    const onBGColorInput = (e: Event): void => {
        bgColor = (e.target as HTMLInputElement).value;
        applyBGColor(bgColor);
    };
    const onBGColorFocusout = (e: Event): void => {
        bgColor = (e.target as HTMLInputElement).value;
        updateBGColor(bgColor);
    };
    const chooseColor = (c: string): BGColor =>
        c === "default"
            ? getComputedStyle(document.documentElement)
                  .getPropertyValue("--bg")
                  .trim()
            : c;

    const reset = (): void => {
        resetSettings();

        const settings = defaultSettings();
        font = settings.font;
        theme = settings.theme;
        bgColor = chooseColor(settings.bgColor);

        console.log(bgColor);
    };

    onMount(() => {
        const settings = getSettings();
        font = settings.font;
        theme = settings.theme;
        bgColor = chooseColor(settings.bgColor);
    });
</script>

<div>
    <header>Settings</header>

    <div class="section">
        <h3>Font</h3>
        <div class="buttons">
            <button
                class="btn"
                class:active={font === "sans-serif"}
                onclick={() => changeFont("sans-serif")}
            >
                Sans-serif
            </button>

            <button
                class="btn"
                class:active={font === "serif"}
                onclick={() => changeFont("serif")}
            >
                Serif
            </button>

            <button
                class="btn"
                class:active={font === "monospace"}
                onclick={() => changeFont("monospace")}
            >
                Monospace
            </button>
        </div>
    </div>

    <div class="section">
        <h3>Theme</h3>
        <div class="buttons">
            <button
                class="btn"
                class:active={theme === "system"}
                onclick={() => changeTheme("system")}
            >
                System
            </button>
            <button
                class="btn"
                class:active={theme === "dark"}
                onclick={() => changeTheme("dark")}
            >
                Dark
            </button>
            <button
                class="btn"
                class:active={theme === "light"}
                onclick={() => changeTheme("light")}
            >
                Light
            </button>
        </div>
    </div>

    <div class="section">
        <h3>Background Color</h3>
        <div class="buttons">
            <input
                class="color-input"
                type="color"
                bind:value={bgColor}
                oninput={onBGColorInput}
                onfocusout={onBGColorFocusout}
            />

            <button
                class="btn"
                class:active={bgColor === "default"}
                onclick={() => changeBGColor("default")}
            >
                Default
            </button>
        </div>
    </div>

    <div class="section">
        <div class="buttons">
            <button class="btn" onclick={reset}>Reset</button>
        </div>
    </div>
</div>

<style>
    h3 {
        text-align: center;
    }

    .buttons {
        margin: 5px 0 0 0;
        display: flex;
        align-items: center;
    }

    .section {
        margin: 10px 0;
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .color-input {
        height: 1.5rem;
    }
</style>
