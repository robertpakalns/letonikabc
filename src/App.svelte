<script lang="ts">
    import LoadDocument from "@/components/Menu/LoadDocument.svelte";
    import NewDocument from "@/components/Menu/NewDocument.svelte";
    import About from "@/components/Menu/About.svelte";
    import Menu from "@/components/Menu/Menu.svelte";
    import Reader from "@/components/Reader.svelte";
    import { applySettings } from "@/storage";
    import { parseRoute } from "@/router";
    import { onMount } from "svelte";

    type State = "menu" | "reader" | "load" | "new" | "about";
    let appState: State = $state<State>("menu");
    let skipManual: boolean = $state<boolean>(false);
    let readHash: string = $state("");
    let displayError: string | undefined = $state(undefined);

    const openReader = (skip: boolean) => {
        skipManual = skip;
        appState = "new";
    };

    const newReader = (hash: string, error: string | undefined): void => {
        readHash = hash;
        appState = "reader";
        displayError = error;
    };

    const openLoader = () => {
        appState = "load";
    };

    const aboutLoader = () => {
        appState = "about";
    };

    const menuLoader = () => {
        appState = "menu";
    };

    const changeState = (s: State) => {
        appState = s;
    };

    const parsed = parseRoute();

    if (parsed.route === "reader") {
        readHash = parsed.hash;
    }

    appState = parsed.route;

    onMount(() => {
        applySettings();
    });
</script>

<div class="globalWrapper">
    {#if appState === "menu"}
        <Menu openNew={(skip) => openReader(skip)} {openLoader} {aboutLoader} />
    {:else if appState === "load"}
        <!-- goRead: no error by design -->
        <!-- openReader: no skip option -->
        <LoadDocument
            goBack={() => changeState("menu")}
            goRead={(hash) => newReader(hash, undefined)}
            openReader={() => openReader(false)}
        />
    {:else if appState === "new"}
        <NewDocument
            {skipManual}
            goBack={() => changeState("menu")}
            goRead={newReader}
        />
    {:else if appState === "reader"}
        <Reader {readHash} goBack={() => changeState("menu")} {displayError} />
    {:else if appState === "about"}
        <About goBack={menuLoader} />
    {/if}
</div>

<style>
    .globalWrapper {
        width: 100vw;
        height: 100vh;
        overflow-y: auto;
        background: var(--bg);
        text-align: justify;
    }
</style>
