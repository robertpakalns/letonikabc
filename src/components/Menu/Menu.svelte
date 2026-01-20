<script lang="ts">
    let { openReader } = $props<{
        openReader: (skip: boolean) => void;
    }>();

    let skip = $state(false);
</script>

<div class="centerWrapper">
    <div class="centered">
        <header>Letonika Better Client</header>

        <div class="buttons">
            <div class="btnWithCheckbox">
                <button class="btn" onclick={() => openReader(skip)}>
                    New
                </button>

                <label>
                    <input type="checkbox" bind:checked={skip} />
                    Skip manual
                </label>
            </div>

            <button class="btn">Load</button>
        </div>

        <div class="text">
            <p>
                Source Code:
                <a
                    href="https://github.com/robertpakalns/letonikabc"
                    target="_blank"
                >
                    https://github.com/robertpakalns/letonikabc
                </a>
            </p>
            <p>by robertpakalns</p>
        </div>
    </div>
</div>

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

        input {
            width: fit-content;
        }
    }
</style>
