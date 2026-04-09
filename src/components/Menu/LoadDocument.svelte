<script lang="ts">
    import { deleteMarkdown, deleteMetadata, getAllMetadata } from "@/db";
    import type { MetadataRecord } from "@/db";
    import { navigateToLoad } from "@/router";
    import { onMount } from "svelte";

    let records: MetadataRecord[] = $state([]);
    const { goBack, goRead, openReader } = $props<{
        goBack: () => void;
        goRead: (readHash: string) => void;
        openReader: () => void;
    }>();

    const handleDelete = async (hash: string) => {
        await deleteMarkdown(hash);
        await deleteMetadata(hash);
        records = records.filter((record) => record.hash !== hash);
    };

    onMount(async () => {
        navigateToLoad();

        records = await getAllMetadata();
    });
</script>

<div class="centerWrapper">
    <div class="centered">
        {#if records.length !== 0}
            <table>
                <thead>
                    <tr>
                        <td>#</td>
                        <td>Title</td>
                        <td>Author</td>
                        <td>Created</td>
                        <td>Edited</td>
                        <td></td>
                    </tr>
                </thead>
                <tbody>
                    {#each records as record, i}
                        {@render row(record, i)}
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

{#snippet row(record: MetadataRecord, i: number)}
    <tr>
        <td>{i + 1}</td>
        {#if record.title}
            <td>{record.title}</td>
        {:else}
            <td class="cursive">No title</td>
        {/if}
        {#if record.author}
            <td>{record.author}</td>
        {:else}
            <td class="cursive">No author</td>
        {/if}
        <td>{new Date(record.created_at).toLocaleDateString()}</td>
        <td>{new Date(record.edited_at).toLocaleDateString()}</td>
        <td>
            <button onclick={() => goRead(record.hash)} class="btn">Go</button>
            <button onclick={() => handleDelete(record.hash)} class="btn">
                Delete
            </button>
        </td>
    </tr>
{/snippet}

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

    .cursive {
        font-style: italic;
    }
</style>
