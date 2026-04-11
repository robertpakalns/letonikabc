<script lang="ts">
    export let open = false;
    export let onClose = () => {};

    function handleKey(e: KeyboardEvent) {
        if (e.key === "Escape") onClose();
    }

    $: if (open) {
        window.addEventListener("keydown", handleKey);
    } else {
        window.removeEventListener("keydown", handleKey);
    }
</script>

{#if open}
    <div class="backdrop" onclick={onClose} role="presentation">
        <div
            class="modal"
            role="presentation"
            onclick={(e) => e.stopPropagation()}
        >
            <button onclick={onClose} class="close">×</button>
            <slot />

            <div class="btns">
                <button class="btn" onclick={onClose}>Back</button>
            </div>
        </div>
    </div>
{/if}

<style>
    .backdrop {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 999999;
    }

    .modal {
        position: relative;
        background: color-mix(in srgb, var(--bg) 80%, transparent);
        padding: 1rem;
        border-radius: 8px;
    }

    .close {
        position: absolute;
        top: 7px;
        right: 10px;
        cursor: pointer;
        border: none;
        outline: none;
        background: transparent;
        transition: 0.3s ease;

        &:hover {
            transform: scale(1.3);
        }
    }

    .btns {
        display: flex;
        justify-content: center;

        .btn {
            font-size: 0.7rem;
        }
    }
</style>
