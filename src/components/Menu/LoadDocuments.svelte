<script lang="ts">
    import { onMount } from "svelte";
    import { getAll, type MDRecord } from "../../db";

    let records: MDRecord[] = [];

    onMount(async () => {
        records = await getAll();
    });
</script>

<div class="centerWrapper">
    <div class="centered">
        <ul>
            <!-- Iterate entries for now -->
            {#each records as record, i}
                <li>{i + 1}</li>
            {/each}
        </ul>
    </div>
</div>
