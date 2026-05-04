<script lang="ts">
    import Settings from "@/components/Settings.svelte";
    import Modal from "@/components/Modal.svelte";
    import { navigateToMain } from "@/router";

    const { openNew, openLoader, aboutLoader } = $props<{
        openNew: (skip: boolean) => void;
        openLoader: () => void;
        aboutLoader: () => void;
    }>();

    let skip = $state(false);

    // Modal
    let open = $state(false);

    navigateToMain();
</script>

<div class="centerWrapper">
    <div class="centered">
        <header>
            <img src="./icons/logo.svg" alt="Logo" />
            <span>Letonika Better Client</span>
        </header>

        <div class="buttons">
            <div class="btnWithCheckbox">
                <button class="btn" onclick={() => openNew(skip)}> New </button>

                <label>
                    <input type="checkbox" bind:checked={skip} />
                    Skip manual
                </label>
            </div>

            <button class="btn" onclick={openLoader}>Load</button>
        </div>

        <div class="buttons">
            <button onclick={() => (open = true)} class="btn">Settings</button>
            <button onclick={aboutLoader} class="btn">About</button>
        </div>
    </div>
</div>

<Modal bind:open onClose={() => (open = false)}>
    <Settings />
</Modal>

<style>
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

    header {
        height: 2rem;
        display: flex;
        align-items: center;
        gap: 7px;

        img {
            height: 100%;
        }

        span {
            font-size: 2rem;
        }
    }
</style>
