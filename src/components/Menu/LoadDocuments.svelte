<script lang="ts">
    import { onMount } from "svelte";
    import { getAll, type MDRecord } from "../../db";

    let records: MDRecord[] = $state([]);

    const { goBack, goRead } = $props<{
        goBack: () => void;
        goRead: (readId: number) => void;
    }>();

    onMount(async () => {
        records = await getAll();
    });
</script>

<div>
    <button class="btn" onclick={goBack}>Back</button>
</div>

<div class="centerWrapper">
    <div class="centered">
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
                            <button onclick={() => goRead(i + 1)} class="btn"
                                >Go</button
                            >
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
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
