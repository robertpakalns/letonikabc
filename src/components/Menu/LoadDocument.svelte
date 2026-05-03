<script lang="ts">
    import {
        deleteMarkdown,
        deleteMetadata,
        getAllMetadata,
        updateMetadata,
        deleteHeadings,
    } from "@/db";
    import { exportMarkdown, formatBytes } from "@/utils";
    import type { MetadataRecord } from "@/db";
    import { navigateToMy } from "@/router";
    import { onMount } from "svelte";

    let editingHash: string | null = $state(null);
    let draft: MetadataRecord | null = $state(null);

    const commitEdit = async () => {
        if (!draft || !editingHash) return;

        const updated: MetadataRecord = {
            ...draft,
            edited_at: new Date().toISOString(),
        };

        await updateMetadata(updated);

        records = records.map((r) => (r.hash === updated.hash ? updated : r));

        draft = null;
        editingHash = null;
    };

    let records: MetadataRecord[] = $state([]);
    const { goBack, goRead, openReader } = $props<{
        goBack: () => void;
        goRead: (readHash: string) => void;
        openReader: () => void;
    }>();

    const handleDelete = async (hash: string) => {
        await deleteMarkdown(hash);
        await deleteMetadata(hash);
        await deleteHeadings(hash);
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

    const handleGlobalClick = (event: MouseEvent) => {
        if (!editingHash) return;

        const target = event.target as HTMLElement;

        const editingEl = document.querySelector(
            `[data-editing="${editingHash}"]`,
        );

        if (!editingEl) return;
        if (editingEl.contains(target)) return;

        commitEdit();
    };

    const switchEdit = (record: MetadataRecord) => {
        if (editingHash === record.hash) return;

        if (draft && editingHash) {
            const updated: MetadataRecord = {
                ...draft,
                edited_at: new Date().toISOString(),
            };

            updateMetadata(updated);

            records = records.map((r) =>
                r.hash === updated.hash ? updated : r,
            );
        }

        editingHash = record.hash;
        draft = JSON.parse(JSON.stringify(record));
    };

    onMount(() => {
        navigateToMy();

        const load = async () => {
            records = await getAllMetadata();
        };

        load();

        document.addEventListener("mousedown", handleGlobalClick);

        return () => {
            document.removeEventListener("mousedown", handleGlobalClick);
        };
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
    <tr data-editing={editingHash === record.hash ? record.hash : undefined}>
        <td>{i + 1}</td>
        <td>
            {#if editingHash === record.hash && draft}
                <input type="text" bind:value={draft.title} />
            {:else if record.title}
                {record.title}
            {:else}
                <span class="cursive">No title</span>
            {/if}
        </td>
        <td>
            {#if editingHash === record.hash && draft}
                <input type="text" bind:value={draft.author} />
            {:else if record.author}
                {record.author}
            {:else}
                <span class="cursive">No author</span>
            {/if}
        </td>
        <td>{new Date(record.created_at).toLocaleDateString()}</td>
        <td>{new Date(record.edited_at).toLocaleDateString()}</td>
        <td>
            <div class="btns">
                <button onclick={() => goRead(record.hash)} class="btn-wrap">
                    <img src="./icons/go.svg" alt="Go read" />
                </button>
                <button onclick={() => switchEdit(record)} class="btn-wrap">
                    <img src="./icons/edit.svg" alt="Edit entry" />
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
        white-space: normal;
        word-break: break-word;
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

    input {
        outline: none;
        border: none;
        background: transparent;
        border-bottom: 1px solid gray;
        width: 100%;
    }
</style>
