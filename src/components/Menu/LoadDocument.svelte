<script lang="ts">
    import { onMount } from "svelte";
    import { getAll, type MDRecord } from "../../db";

    let records: MDRecord[] = $state([]);

    const { goBack, goRead, openReader } = $props<{
        goBack: () => void;
        goRead: (readHash: string) => void;
        openReader: () => void;
    }>();

    onMount(async () => {
        records = await getAll();
    });
</script>

<div class="centerWrapper">
    <div class="centered">
        {#if records.length !== 0}
            <table>
                <thead>
                    <tr>
                        <td>#</td>
                        <td></td>
                    </tr>
                </thead>
                <tbody>
                    {#each records as record, i}
                        <tr>
                            <td>{i + 1}</td>
                            <td>
                                <button
                                    onclick={() => goRead(record.hash)}
                                    class="btn"
                                >
                                    Go
                                </button>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {:else}
            <header>No documents found</header>
        {/if}

        <div class="buttons">
            <button class="btn" onclick={openReader}>New</button>
            <button class="btn" onclick={goBack}>Back</button>
        </div>
    </div>
</div>

<style>
    table {
        border-collapse: collapse;
    }

    thead {
        font-weight: 900;
    }

    td {
        padding: 6px 10px;
    }
</style>
