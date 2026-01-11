<script lang="ts">
    import Menu from "./components/Menu/Menu.svelte";
    import Reader from "./components/Reader.svelte";

    type State = "menu" | "reader";
    let state: State = "menu";

    const changeState = (s: State) => {
        state = s;
    };
</script>

<div class="globalWrapper">
    {#if state === "menu"}
        <Menu openReader={() => changeState("reader")} />
    {:else}
        <Reader goBack={() => changeState("menu")} />
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
