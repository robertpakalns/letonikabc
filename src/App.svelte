<script lang="ts">
    import LoadDocuments from "./components/Menu/LoadDocuments.svelte";
    import NewDocument from "./components/Menu/NewDocument.svelte";
    import Menu from "./components/Menu/Menu.svelte";
    import Reader from "./components/Reader.svelte";

    type State = "menu" | "reader" | "load" | "new";
    let state: State = "menu";
    let skipManual = false;
    let readHash: string;

    const openReader = (skip: boolean) => {
        skipManual = skip;
        state = "new";
    };

    const newReader = (hash: string): void => {
        readHash = hash;
        state = "reader";
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
        <LoadDocuments goBack={() => changeState("menu")} goRead={newReader} />
    {:else if state === "new"}
        <NewDocument
            {skipManual}
            goBack={() => changeState("menu")}
            goRead={newReader}
        />
    {:else if state === "reader"}
        <Reader {readHash} goBack={() => changeState("menu")} />
    {/if}
</div>

<style>
    .globalWrapper {
        width: 100vw;
        height: 100vh;
        overflow-y: auto;
        background: #323339; /* Dark mode for now */
        color: #dbdcde;
        text-align: justify;
    }
</style>
