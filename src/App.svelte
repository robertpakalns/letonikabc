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

<!--
     TODO:
     1. Handle file open in a separate component
     2. Save data in IndexedDB
     3. Save parsed document ID
     4. Open reader with the ID as prop
     5. Get markdown using document ID from IndexedDB
-->

<div class="globalWrapper">
    {#if state === "menu"}
        <Menu openNew={(skip) => openReader(skip)} {openLoader} />
    {:else if state === "load"}
        <LoadDocuments />
    {:else if state === "new"}
        <NewDocument
            {skipManual}
            goBack={() => changeState("menu")}
            goRead={(id) => newReader(id)}
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
        background: #323339;
        color: #dbdcde;
        text-align: justify;
    }
</style>
