<script lang="ts">
    import LoadDocument from "./components/Menu/LoadDocument.svelte";
    import NewDocument from "./components/Menu/NewDocument.svelte";
    import Menu from "./components/Menu/Menu.svelte";
    import Reader from "./components/Reader.svelte";

    type State = "menu" | "reader" | "load" | "new";
    let state: State = "menu";
    let skipManual = false;
    let readHash: string;
    let displayError: string | undefined;

    const openReader = (skip: boolean) => {
        skipManual = skip;
        state = "new";
    };

    const newReader = (hash: string, error: string | undefined): void => {
        readHash = hash;
        state = "reader";
        displayError = error;
    };

    const openLoader = () => {
        state = "load";
    };

    const changeState = (s: State) => {
        state = s;
    };
</script>

<div class="globalWrapper">
    {#if state === "menu"}
        <Menu openNew={(skip) => openReader(skip)} {openLoader} />
    {:else if state === "load"}
        <!-- goRead: no error by design -->
        <!-- openReader: no skip option -->
        <LoadDocument
            goBack={() => changeState("menu")}
            goRead={(hash) => newReader(hash, undefined)}
            openReader={() => openReader(false)}
        />
    {:else if state === "new"}
        <NewDocument
            {skipManual}
            goBack={() => changeState("menu")}
            goRead={newReader}
        />
    {:else if state === "reader"}
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
