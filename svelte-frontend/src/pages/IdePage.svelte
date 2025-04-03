<script lang="ts">
    import { onMount } from "svelte";
    import {
        Content,
        DataTable,
        Button,
        InlineNotification,
        Modal,
        DataTableSkeleton,
    } from "carbon-components-svelte";
    import { Add, Edit, TrashCan } from "carbon-icons-svelte";
    import type { NFeIdentification } from "../types/nfeTypes";
    import ItemForm from "../components/ItemForm.svelte";
    import {
        fetchIdentifications,
        createIdentification,
        updateIdentification,
        deleteIdentification,
    } from "../services/api";

    export let navigateTo: (path: string) => void;

    // State variables
    let items: NFeIdentification[] = [];
    let editingItem: NFeIdentification | null = null;
    let isFormModalOpen = false;
    let isDeleteModalOpen = false;
    let itemToDelete: string | null = null;
    let loading = true;
    let error: string | null = null;
    let rows: Array<{
        id: string;
        cUF: string;
        natOp: string;
        nNF: string;
        dhEmi: string;
        actions: string;
    }> = [];

    // Table headers for Carbon DataTable
    const headers = [
        { key: "id", value: "ID", empty: false },
        { key: "cUF", value: "UF Code", empty: false },
        { key: "natOp", value: "Nature of Operation", empty: false },
        { key: "nNF", value: "NFe Number", empty: false },
        { key: "dhEmi", value: "Issue Date", empty: false },
        { key: "actions", value: "Actions", empty: false },
    ];

    // Prepare data for the table rows
    $: {
        rows = items
            .map((item) => {
                if (!item.internal_key) {
                    console.error("Item missing internal_key:", item);
                    return null;
                }
                return {
                    id: item.internal_key,
                    cUF: item.cUF || "",
                    natOp: item.natOp || "",
                    nNF: item.nNF || "",
                    dhEmi: item.dhEmi ? formatDate(item.dhEmi) : "",
                    actions: `actions-${item.internal_key}`,
                };
            })
            .filter((row): row is NonNullable<typeof row> => row !== null);
    }

    // Load data on component mount
    onMount(async () => {
        try {
            loading = true;
            items = await fetchIdentifications();
            console.log("Fetched items:", items);
        } catch (err) {
            error = err instanceof Error ? err.message : "Failed to load items";
        } finally {
            loading = false;
        }
    });

    // Add a function to handle navigation after successful operations
    function handleOperationSuccess() {
        // Refresh the current page
        navigateTo("/ide");
    }

    // Update the success handlers to use navigation
    async function handleCreate(event: CustomEvent) {
        try {
            const formData = event.detail;
            console.log("Received form data:", formData);
            const newItem = await createIdentification(formData);
            items = [...items, newItem];
            isFormModalOpen = false;
            handleOperationSuccess();
        } catch (err) {
            error =
                err instanceof Error ? err.message : "Failed to create item";
        }
    }

    async function handleUpdate(event: CustomEvent) {
        if (!editingItem) return;
        try {
            const formData = {
                ...event.detail,
                internal_key: editingItem.internal_key,
                created_at: editingItem.created_at,
                updated_at: editingItem.updated_at,
            };
            console.log("Updating item with data:", formData);
            const updatedItem = await updateIdentification(
                editingItem.internal_key,
                formData,
            );
            items = items.map((item) =>
                item.internal_key === updatedItem.internal_key
                    ? updatedItem
                    : item,
            );
            isFormModalOpen = false;
            editingItem = null;
            handleOperationSuccess();
        } catch (err) {
            error =
                err instanceof Error ? err.message : "Failed to update item";
        }
    }

    async function handleDelete() {
        if (!itemToDelete) return;
        try {
            await deleteIdentification(itemToDelete);
            items = items.filter((item) => item.internal_key !== itemToDelete);
            isDeleteModalOpen = false;
            itemToDelete = null;
            handleOperationSuccess();
        } catch (err) {
            error =
                err instanceof Error ? err.message : "Failed to delete item";
        }
    }

    function handleEdit(item: NFeIdentification) {
        editingItem = item;
        isFormModalOpen = true;
    }

    function handleDeleteClick(item: NFeIdentification) {
        itemToDelete = item.internal_key;
        isDeleteModalOpen = true;
    }

    function formatDate(dateString: string) {
        return new Date(dateString).toLocaleString();
    }
</script>

<Content>
    <div class="page-container">
        <div class="header">
            <h1>NFe Identifications</h1>
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
            <DataTable {rows} {headers} sortable zebra size="medium">
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
    </div>
</Content>

<style>
    .page-container {
        padding: 2rem;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }

    .actions {
        display: flex;
        gap: 0.5rem;
    }

    :global(.bx--data-table) {
        background-color: var(--dark-bg);
    }

    :global(.bx--data-table-header) {
        background-color: var(--dark-bg);
    }

    :global(.bx--data-table-row) {
        background-color: var(--dark-bg);
    }

    :global(.bx--data-table-row:hover) {
        background-color: var(--dark-bg-light);
    }

    :global(.bx--data-table-cell) {
        color: var(--white);
    }

    :global(.bx--data-table-header-cell) {
        color: var(--light-gray);
    }

    :global(.bx--data-table--zebra tbody tr:nth-child(odd)) {
        background-color: var(--dark-bg-light);
    }

    :global(.bx--data-table--zebra tbody tr:nth-child(odd):hover) {
        background-color: var(--dark-bg);
    }
</style>
