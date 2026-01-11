<script lang="ts">
    let { openReader } = $props();
</script>

<div class="menuWrapper">
    <div class="centered">
        <header>Letonika Better Client</header>

        <div class="buttons">
            <button class="btn" onclick={openReader}>New</button>
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
    header {
        font-size: 1.5rem;
    }

    .menuWrapper {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100%;
        gap: 10px;
    }

    .centered {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        p {
            width: 100%;
            text-align: left;
        }
    }

    .buttons {
        margin: 10px 0;
        display: flex;
        gap: 5px;
    }

    .text {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }
</style>
