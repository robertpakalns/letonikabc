<script lang="ts">
    import LoadDocument from "@/components/Menu/LoadDocument.svelte";
    import NewDocument from "@/components/Menu/NewDocument.svelte";
    import Menu from "@/components/Menu/Menu.svelte";
    import Reader from "@/components/Reader.svelte";
    import { parseRoute } from "@/router";

    type State = "menu" | "reader" | "load" | "new";
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

    const changeState = (s: State) => {
        appState = s;
    };

    const parsed = parseRoute();

    if (parsed.route === "reader") {
        readHash = parsed.hash;
    }

    appState = parsed.route;
</script>

<div class="globalWrapper">
    {#if appState === "menu"}
        <Menu openNew={(skip) => openReader(skip)} {openLoader} />
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
