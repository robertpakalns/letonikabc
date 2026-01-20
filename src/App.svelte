<script lang="ts">
    import Menu from "./components/Menu/Menu.svelte";
    import Reader from "./components/Reader.svelte";
    import LoadDocuments from "./components/Menu/LoadDocuments.svelte";

    type State = "menu" | "reader" | "load";
    let state: State = "menu";
    let skipManual = false;

    const openReader = (skip: boolean) => {
        skipManual = skip;
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
        <Menu {openReader} {openLoader} />
    {:else if state === "load"}
        <LoadDocuments />
    {:else}
        <Reader {skipManual} goBack={() => changeState("menu")} />
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
