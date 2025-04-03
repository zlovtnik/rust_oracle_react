<script lang="ts">
    import { Button } from "carbon-components-svelte";

    export let navigateTo: (path: string) => void;
</script>

<div class="nfe-page">
    <div class="page-section-header">
        <div class="section-title">
            <h2>NFe Management</h2>
            <div class="accent-line"></div>
        </div>
    </div>

    <div class="card-container">
        <div class="table-header">
            <Button
                kind="primary"
                size="small"
                class="add-button"
                on:click={() => navigateTo("/ide")}
            >
                Add NFe
            </Button>
        </div>
        <!-- Add NFe table content here -->
    </div>
</div>
