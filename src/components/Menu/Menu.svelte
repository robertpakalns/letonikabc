<script lang="ts">
    import Settings from "@/components/Settings.svelte";
    import Modal from "@/components/Modal.svelte";
    import { navigateToMain } from "@/router";

    let { openNew, openLoader } = $props<{
        openNew: (skip: boolean) => void;
        openLoader: () => void;
    }>();

    let skip = $state(false);

    // Modal
    let open = $state(false);

    navigateToMain();
</script>

<div class="centerWrapper">
    <div class="centered">
        <header>Letonika Better Client</header>

        <div class="buttons">
            <div class="btnWithCheckbox">
                <button class="btn" onclick={() => openNew(skip)}> New </button>

                <label>
                    <input type="checkbox" bind:checked={skip} />
                    Skip manual
                </label>
            </div>

            <button class="btn" onclick={openLoader}>Load</button>

            <button onclick={() => (open = true)} class="btn">Settings</button>
        </div>

        <div class="text">
            <p>
                Source code:
                <a
                    href="https://github.com/robertpakalns/letonikabc"
                    target="_blank"
                >
                    https://github.com/robertpakalns/letonikabc
                </a>
            </p>
        </div>
    </div>
</div>

<Modal bind:open onClose={() => (open = false)}>
    <Settings />
</Modal>

<style>
    .text {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .btnWithCheckbox {
        display: flex;
        flex-direction: column;
        gap: 5px;

        label {
            cursor: pointer;
            user-select: none;
        }

        input {
            cursor: pointer;
        }
    }
</style>
