<script lang="ts">
    import LoadDocuments from "./components/Menu/LoadDocuments.svelte";
    import NewDocument from "./components/Menu/NewDocument.svelte";
    import Menu from "./components/Menu/Menu.svelte";
    import Reader from "./components/Reader.svelte";

    type State = "menu" | "reader" | "load" | "new";
    let state: State = "menu";
    let skipManual = false;
    let readId: number;

    const openReader = (skip: boolean) => {
        skipManual = skip;
        state = "new";
    };

    const newReader = (id: number): void => {
        readId = id;
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
        <Reader {readId} goBack={() => changeState("menu")} />
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
