<script lang="ts">
    import {
        Button,
        Search,
        DataTable,
        DataTableSkeleton,
        InlineNotification,
        Modal,
        TextInput,
        Select,
        SelectItem,
    } from "carbon-components-svelte";
    import { Filter, Add, Edit, TrashCan, Close } from "carbon-icons-svelte";
    import { fade } from "svelte/transition";
    import ItemForm from "../components/ItemForm.svelte";
    import { onMount } from "svelte";
    import {
        fetchIdentifications,
        createIdentification,
        updateIdentification,
        deleteIdentification,
    } from "../services/api";
    import type { NFeIdentification } from "../types/nfeTypes";

    export let navigateTo: (path: string) => void;

    interface FormData {
        natOp: string;
        cUF: string;
        cNF: string;
        mod_: string;
        serie: string;
        nNF: string;
        dEmi: string;
        dhEmi: string;
        tpNF: string;
        idDest: string;
        cMunFG: string;
        tpImp: string;
        tpEmis: string;
        cDV: string;
        tpAmb: string;
        finNFe: string;
        indFinal: string;
        indPres: string;
        procEmi: string;
        verProc: string;
        created_at: string;
        updated_at: string;
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

    // Filter state
    let filters = {
        natOp: "",
        nNF: "",
        tpNF: "",
        dhEmi: "",
        cUF: "",
        cNF: "",
        mod_: "",
        serie: "",
        dEmi: "",
        idDest: "",
        cMunFG: "",
        tpImp: "",
        tpEmis: "",
        cDV: "",
        tpAmb: "",
        finNFe: "",
        indFinal: "",
        indPres: "",
        procEmi: "",
        verProc: "",
    } as FormData;

    interface TableRow {
        id: string;
        internal_key: string;
        natOp: string;
        nNF: string;
        dhEmi: string;
        tpNF: string;
    }

    let headers = [
        { key: "internal_key", value: "Internal Key", empty: false },
        { key: "natOp", value: "Nature of Operation", empty: false },
        { key: "nNF", value: "NF Number", empty: false },
        { key: "dhEmi", value: "Issue Date", empty: false },
        { key: "tpNF", value: "Type", empty: false },
        { key: "actions", value: "Actions", empty: false },
    ];

    $: filteredItems = items.filter((item) => {
        const matchesNatOp =
            !filters.natOp ||
            item.natOp.toLowerCase().includes(filters.natOp.toLowerCase());
        const matchesNNF = !filters.nNF || item.nNF.includes(filters.nNF);
        const matchesTpNF = !filters.tpNF || item.tpNF === filters.tpNF;
        const matchesDhEmi =
            !filters.dhEmi || item.dhEmi.includes(filters.dhEmi);

        return matchesNatOp && matchesNNF && matchesTpNF && matchesDhEmi;
    });

    $: filteredRows = filteredItems.map(
        (item) =>
            ({
                id: item.internal_key,
                internal_key: item.internal_key,
                natOp: item.natOp,
                nNF: item.nNF,
                dhEmi: new Date(item.dhEmi).toLocaleDateString(),
                tpNF: item.tpNF === "1" ? "Output" : "Input",
            }) as TableRow,
    );

    function clearFilters(): void {
        filters = {
            natOp: "",
            nNF: "",
            tpNF: "",
            dhEmi: "",
            cUF: "",
            cNF: "",
            mod_: "",
            serie: "",
            dEmi: "",
            idDest: "",
            cMunFG: "",
            tpImp: "",
            tpEmis: "",
            cDV: "",
            tpAmb: "",
            finNFe: "",
            indFinal: "",
            indPres: "",
            procEmi: "",
            verProc: "",
        } as FormData;
    }

    onMount(async () => {
        try {
            loading = true;
            items = await fetchIdentifications();
        } catch (error: unknown) {
            const err = error as Error;
            error = err.message || "Failed to load items";
        } finally {
            loading = false;
        }
    });

    async function handleCreate(
        event: CustomEvent<
            Omit<
                NFeIdentification,
                "internal_key" | "created_at" | "updated_at"
            >
        >,
    ): Promise<void> {
        try {
            const newItem = await createIdentification({
                ...event.detail,
                created_at: new Date().toISOString(),
                updated_at: new Date().toISOString(),
            });
            items = [...items, newItem];
            isFormModalOpen = false;
        } catch (error: unknown) {
            const err = error as Error;
            error = err.message || "Failed to create item";
        }
    }

    async function handleUpdate(
        event: CustomEvent<
            Omit<
                NFeIdentification,
                "internal_key" | "created_at" | "updated_at"
            >
        >,
    ): Promise<void> {
        if (!editingItem?.internal_key) return;
        try {
            const updatedItem = await updateIdentification(
                editingItem.internal_key,
                {
                    ...editingItem,
                    ...event.detail,
                    updated_at: new Date().toISOString(),
                },
            );
            items = items.map((item) =>
                item.internal_key === updatedItem.internal_key
                    ? updatedItem
                    : item,
            );
            isFormModalOpen = false;
            editingItem = null;
        } catch (error: unknown) {
            const err = error as Error;
            error = err.message || "Failed to update item";
        }
    }

    async function handleDelete(): Promise<void> {
        if (!itemToDelete) return;
        try {
            await deleteIdentification(itemToDelete);
            items = items.filter((item) => item.internal_key !== itemToDelete);
            isDeleteModalOpen = false;
            itemToDelete = null;
        } catch (error: unknown) {
            const err = error as Error;
            error = err.message || "Failed to delete item";
        }
    }

    function handleEdit(row: TableRow): void {
        const item = items.find((i) => i.internal_key === row.id);
        if (item) {
            editingItem = item;
            isFormModalOpen = true;
        }
    }

    function handleDeleteClick(row: TableRow): void {
        itemToDelete = row.id;
        isDeleteModalOpen = true;
    }
</script>

<div class="page-container">
    <div class="header">
        <h1>NFe Identifications</h1>
        <div class="header-actions">
            <Button
                kind="ghost"
                icon={Filter}
                on:click={() => (isFilterPanelOpen = !isFilterPanelOpen)}
            >
                {#if isFilterPanelOpen}
                    Close Filters
                {:else}
                    Open Filters
                {/if}
            </Button>
            <div class="search-input">
                <Search
                    placeholder="Search..."
                    bind:value={searchQuery}
                    size="lg"
                />
            </div>
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

    {#if isFilterPanelOpen}
        <div class="filter-panel" transition:fade>
            <div class="filter-header">
                <h2>Filters</h2>
                <Button
                    kind="ghost"
                    icon={Close}
                    iconDescription="Clear Filters"
                    on:click={clearFilters}
                >
                    Clear
                </Button>
            </div>
            <div class="filter-content">
                <div class="filter-group">
                    <TextInput
                        labelText="Nature of Operation"
                        placeholder="Filter by nature of operation..."
                        bind:value={filters.natOp}
                    />
                </div>
                <div class="filter-group">
                    <TextInput
                        labelText="NF Number"
                        placeholder="Filter by NF number..."
                        bind:value={filters.nNF}
                    />
                </div>
                <div class="filter-group">
                    <Select labelText="Type" selected={filters.tpNF}>
                        <SelectItem value="" text="All" />
                        <SelectItem value="1" text="Output" />
                        <SelectItem value="0" text="Input" />
                    </Select>
                </div>
                <div class="filter-group">
                    <TextInput
                        labelText="Issue Date"
                        placeholder="Filter by issue date..."
                        type="date"
                        bind:value={filters.dhEmi}
                    />
                </div>
            </div>
        </div>
    {/if}

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
                                on:click={() => handleEdit(row)}
                            >
                                Edit
                            </Button>
                            <Button
                                kind="ghost"
                                size="small"
                                icon={TrashCan}
                                on:click={() => handleDeleteClick(row)}
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
</div>

<style>
    @import url("https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap");

    :global(body) {
        font-family:
            "Inter",
            -apple-system,
            BlinkMacSystemFont,
            system-ui,
            sans-serif;
        background: #e6e6fa;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }

    /* Page Layout */
    .page-container {
        padding: 24px;
        min-height: 100vh;
        background: #e6e6fa;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin: -24px -24px 32px -24px;
        padding: 24px;
        background: #004d40;
    }

    h1 {
        font-size: 24px;
        font-weight: 600;
        color: #32cd32;
        margin: 0;
        letter-spacing: -0.025em;
    }

    .header-actions {
        display: flex;
        gap: 12px;
        align-items: center;
    }

    /* Button Styles */
    :global(.bx--btn) {
        height: 40px;
        border-radius: 8px;
        font-weight: 500;
        font-size: 14px;
        padding: 0 16px;
        transition: all 0.15s ease;
    }

    :global(.bx--btn--ghost) {
        background: #00695c;
        border: 1px solid #32cd32;
        color: #90ee90;
        display: flex;
        align-items: center;
        gap: 8px;
        font-size: 14px;
    }

    :global(.bx--btn--ghost:hover) {
        background: #004d40;
        border-color: #32cd32;
        color: #32cd32;
    }

    :global(.bx--btn--primary) {
        background: #32cd32;
        border: none;
        color: #004d40;
        display: flex;
        align-items: center;
        gap: 8px;
    }

    :global(.bx--btn--primary:hover) {
        background: #90ee90;
    }

    /* Search Input */
    :global(.bx--search) {
        width: 320px;
    }

    :global(.bx--search-input) {
        background: #6a0dad;
        border: 1px solid #32cd32;
        border-radius: 8px;
        height: 40px;
        color: #90ee90;
    }

    :global(.bx--search-input::placeholder) {
        color: #90ee90;
    }

    /* Filter Panel */
    .filter-panel {
        background: #4b0082;
        border: 1px solid #32cd32;
        padding: 24px;
        margin-bottom: 24px;
        border-radius: 12px;
    }

    .filter-content {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
        gap: 20px;
    }

    .filter-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 20px;
        padding-bottom: 16px;
        border-bottom: 1px solid #32cd32;
    }

    .filter-header h2 {
        color: #32cd32;
        margin: 0;
        font-size: 18px;
        font-weight: 500;
    }

    /* Form Inputs */
    :global(.bx--text-input),
    :global(.bx--select-input) {
        background: #6a0dad;
        border: 1px solid #32cd32;
        height: 40px;
        border-radius: 8px;
        color: #90ee90;
        padding: 0 12px;
    }

    :global(.bx--text-input::placeholder) {
        color: #90ee90;
    }

    :global(.bx--text-input:focus),
    :global(.bx--select-input:focus) {
        border-color: #32cd32;
        box-shadow: 0 0 0 3px rgba(50, 205, 50, 0.2);
        outline: none;
    }

    :global(.bx--label) {
        color: #32cd32;
        font-weight: 500;
        margin-bottom: 6px;
        font-size: 14px;
    }

    :global(.bx--select-input option) {
        background: #00695c;
        color: #90ee90;
    }

    /* Data Table */
    :global(.bx--data-table-container) {
        background: #4b0082;
        border-radius: 12px;
        overflow: hidden;
        border: 1px solid #32cd32;
    }

    :global(.bx--data-table) {
        width: 100%;
        border-collapse: separate;
        border-spacing: 0;
        font-size: 14px;
        background: #4b0082;
    }

    :global(.bx--data-table th) {
        background: #6a0dad;
        color: #32cd32;
        font-weight: 500;
        text-align: left;
        padding: 12px 16px;
        font-size: 12px;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        border-bottom: 2px solid #32cd32;
    }

    :global(.bx--data-table td) {
        padding: 12px 16px;
        color: #90ee90;
        background: #4b0082;
        font-size: 14px;
        line-height: 1.5;
        border-bottom: 1px solid #32cd32;
    }

    :global(.bx--data-table tr:nth-child(odd) td) {
        background: #4b0082;
    }

    :global(.bx--data-table tr:nth-child(even) td) {
        background: #6a0dad;
    }

    :global(.bx--data-table tbody tr:hover td) {
        background: #9400d3 !important;
        color: #90ee90;
    }

    :global(.bx--table-toolbar) {
        background: #6a0dad;
        padding: 16px;
        border-bottom: 1px solid #32cd32;
    }

    /* Table Actions */
    .actions {
        display: flex;
        gap: 8px;
    }

    :global(.bx--data-table tbody tr:hover td .actions button) {
        background: #9400d3;
        color: #90ee90;
        border-color: #32cd32;
    }

    :global(.bx--data-table tbody tr:hover td .actions button:hover) {
        background: #8b008b;
        color: #90ee90;
        border-color: #32cd32;
    }

    :global(.bx--data-table tbody tr:hover td .actions button svg) {
        fill: #90ee90;
    }

    /* Checkbox */
    :global(.bx--checkbox) {
        border-color: #32cd32;
    }

    :global(.bx--checkbox:checked) {
        background: #32cd32;
        border-color: #32cd32;
    }

    /* Data Table Header */
    :global(.bx--data-table-header) {
        padding: 0;
    }
</style>
