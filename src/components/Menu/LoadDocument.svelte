<script lang="ts">
    import { deleteMarkdown, deleteMetadata, getAllMetadata } from "@/db";
    import { exportMarkdown, formatBytes } from "@/utils";
    import type { MetadataRecord } from "@/db";
    import { navigateToMy } from "@/router";
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

    const getPercent = (record: MetadataRecord): string => {
        if (!record.size_before) return "0%";
        const percent =
            ((record.size_before - record.size_after) / record.size_before) *
            100;
        return `${percent.toFixed(2)}%`;
    };

    const getRecordInfo = (record: MetadataRecord): string => {
        return `Before: ${formatBytes(record.size_before)}\nAfter: ${formatBytes(record.size_after)} (${getPercent(record)})`;
    };

    onMount(async () => {
        navigateToMy();

        records = await getAllMetadata();
    });
</script>

<div class="centerWrapper">
    <div class="centered">
        <header class="pad">My documents</header>

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
            <p class="center">No documents found</p>
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
            <div class="btns">
                <button onclick={() => goRead(record.hash)} class="btn-wrap">
                    <img src="./icons/go.svg" alt="Go read" />
                </button>
                <button
                    onclick={() => handleDelete(record.hash)}
                    class="btn-wrap"
                >
                    <img src="./icons/delete.svg" alt="Delete entry" />
                </button>
                <button title={getRecordInfo(record)} class="btn-wrap">
                    <img src="./icons/info.svg" alt="Information about entry" />
                </button>
                <button
                    onclick={() => exportMarkdown(record.hash)}
                    class="btn-wrap"
                >
                    <img src="./icons/export.svg" alt="Export markdown" />
                </button>
            </div>
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
        white-space: normal; /* allow wrapping */
        word-break: break-word; /* break long words */
    }

    .cursive {
        font-style: italic;
    }

    .pad {
        padding: 20px 0;
    }

    .center {
        text-align: center;
    }

    .btn-wrap {
        background: transparent;
        border: none;
        outline: none;
        height: 20px;
        cursor: pointer;
        margin: 4px;

        img {
            height: 100%;
        }
    }

    .btns {
        @media (max-width: 600px) {
            display: grid;
            grid-template-columns: repeat(2, 1fr);
        }
    }
</style>
