<script lang="ts">
    import {
        Button,
        Search,
        DataTable,
        DataTableSkeleton,
        InlineNotification,
        Modal,
    } from "carbon-components-svelte";
    import { Filter, Add, Edit, TrashCan } from "carbon-icons-svelte";
    import { fade } from "svelte/transition";
    import ItemForm from "../components/ItemForm.svelte";
    import { onMount } from "svelte";
    import type { NFeIdentification } from "../types/nfeTypes";

    export let navigateTo: (path: string) => void;

    interface TableRow {
        id: string;
        internal_key: string;
        natOp: string;
        nNF: string;
        dhEmi: string;
        tpNF: string;
    }

    let searchQuery = "";
    let isFilterPanelOpen = false;
    let editingItem: NFeIdentification | null = null;
    let isFormModalOpen = false;
    let isDeleteModalOpen = false;
    let itemToDelete: string | null = null;
    let error: string | null = null;
    let loading = false;
    let items: NFeIdentification[] = [];
    let filteredRows: TableRow[] = [];
    let headers = [
        { key: "internal_key", value: "Internal Key", empty: false },
        { key: "natOp", value: "Nature of Operation", empty: false },
        { key: "nNF", value: "NF Number", empty: false },
        { key: "dhEmi", value: "Issue Date", empty: false },
        { key: "tpNF", value: "Type", empty: false },
        { key: "actions", value: "Actions", empty: false },
    ];

    onMount(async () => {
        try {
            loading = true;
            // Fetch items from your API
            // items = await fetchItems();
            // Update filteredRows based on items
        } catch (err) {
            error = err instanceof Error ? err.message : "Failed to load items";
        } finally {
            loading = false;
        }
    });

    async function handleCreate(): Promise<void> {
        try {
            // Create item through your API
            // const newItem = await createItem(event.detail);
            // items = [...items, newItem];
            isFormModalOpen = false;
            navigateTo("/ide"); // Refresh the page
        } catch (err) {
            error =
                err instanceof Error ? err.message : "Failed to create item";
        }
    }

    async function handleUpdate(): Promise<void> {
        if (!editingItem) return;
        try {
            // Update item through your API
            // const updatedItem = await updateItem(editingItem.internal_key, {
            //     ...editingItem,
            //     ...event.detail,
            //     updated_at: new Date().toISOString()
            // });
            // items = items.map((item) =>
            //     item.internal_key === updatedItem.internal_key ? updatedItem : item
            // );
            isFormModalOpen = false;
            editingItem = null;
            navigateTo("/ide"); // Refresh the page
        } catch (err) {
            error =
                err instanceof Error ? err.message : "Failed to update item";
        }
    }

    async function handleDelete(): Promise<void> {
        if (!itemToDelete) return;
        try {
            // Delete item through your API
            // await deleteItem(itemToDelete);
            // items = items.filter((item) => item.internal_key !== itemToDelete);
            isDeleteModalOpen = false;
            itemToDelete = null;
            navigateTo("/ide"); // Refresh the page
        } catch (err) {
            error =
                err instanceof Error ? err.message : "Failed to delete item";
        }
    }

    function handleEdit(item: NFeIdentification): void {
        editingItem = item;
        isFormModalOpen = true;
    }

    function handleDeleteClick(item: NFeIdentification): void {
        itemToDelete = item.internal_key;
        isDeleteModalOpen = true;
    }
</script>

<div class="ide-page">
    <div class="header">
        <h1>NFe Identifications</h1>
        <div class="header-actions">
            <Button
                kind="ghost"
                icon={Filter}
                on:click={() => (isFilterPanelOpen = !isFilterPanelOpen)}
                class="filter-button"
            >
                {#if isFilterPanelOpen}
                    Close Filters
                {:else}
                    Open Filters
                {/if}
            </Button>
            <Search
                placeholder="Search..."
                bind:value={searchQuery}
                size="lg"
                class="search-input"
            />
            <Button
                kind="primary"
                icon={Add}
                on:click={() => {
                    editingItem = null;
                    isFormModalOpen = true;
                }}
            >
                Add Identification
            </Button>
        </div>
    </div>

    {#if error}
        <InlineNotification
            kind="error"
            title="Error"
            subtitle={error}
            on:close={() => (error = null)}
        />
    {/if}

    {#if loading}
        <DataTableSkeleton />
    {:else}
        <div class="table-container">
            <DataTable
                rows={filteredRows}
                {headers}
                sortable
                zebra
                size="medium"
            >
                <svelte:fragment slot="cell" let:cell let:row>
                    {#if cell.key === "actions"}
                        <div class="actions">
                            <Button
                                kind="ghost"
                                size="small"
                                icon={Edit}
                                on:click={() => {
                                    console.log(
                                        "Edit button clicked for row:",
                                        row,
                                    );
                                    const item = items.find(
                                        (i) => i.internal_key === row.id,
                                    );
                                    if (item) handleEdit(item);
                                }}
                            >
                                Edit
                            </Button>
                            <Button
                                kind="ghost"
                                size="small"
                                icon={TrashCan}
                                on:click={() => {
                                    console.log(
                                        "Delete button clicked for row:",
                                        row,
                                    );
                                    const item = items.find(
                                        (i) => i.internal_key === row.id,
                                    );
                                    if (item) handleDeleteClick(item);
                                }}
                            >
                                Delete
                            </Button>
                        </div>
                    {:else}
                        {cell.value}
                    {/if}
                </svelte:fragment>
            </DataTable>
        </div>
    {/if}

    <ItemForm
        open={isFormModalOpen}
        initialValues={editingItem || undefined}
        on:finish={editingItem ? handleUpdate : handleCreate}
        on:cancel={() => {
            isFormModalOpen = false;
            editingItem = null;
        }}
    />

    <Modal
        open={isDeleteModalOpen}
        modalHeading="Delete Identification"
        primaryButtonText="Delete"
        secondaryButtonText="Cancel"
        on:click:button--primary={handleDelete}
        on:click:button--secondary={() => {
            isDeleteModalOpen = false;
            itemToDelete = null;
        }}
        on:close={() => {
            isDeleteModalOpen = false;
            itemToDelete = null;
        }}
    >
        <p>Are you sure you want to delete this identification?</p>
    </Modal>

    {#if isFilterPanelOpen}
        <div class="filter-panel" transition:fade>
            <h2>Filters</h2>
            <!-- ... existing filter panel content ... -->
        </div>
    {/if}
</div>

<style>
    .ide-page {
        padding: 1rem;
        height: 100%;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
        background-color: #ffffff;
        border-radius: 8px;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    }

    .header h1 {
        margin: 0;
        font-size: 1.5rem;
        color: #333333;
    }

    .header-actions {
        display: flex;
        gap: 1rem;
        align-items: center;
    }

    .table-container {
        flex: 1;
        background-color: #ffffff;
        border-radius: 8px;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
        overflow: auto;
    }

    .filter-panel {
        position: fixed;
        top: 0;
        right: 0;
        width: 300px;
        height: 100vh;
        background-color: #ffffff;
        box-shadow: -2px 0 5px rgba(0, 0, 0, 0.1);
        padding: 1rem;
        z-index: 1000;
    }

    .filter-panel h2 {
        margin: 0 0 1rem 0;
        font-size: 1.25rem;
        color: #333333;
        padding-bottom: 0.5rem;
        border-bottom: 1px solid #e0e0e0;
    }
</style>
