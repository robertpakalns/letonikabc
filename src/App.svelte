<script lang="ts">
    import Reader from "./components/Reader.svelte";
</script>

<div class="globalWrapper">
    <Reader />
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
