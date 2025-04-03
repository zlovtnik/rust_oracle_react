<script lang="ts">
    import {
        Button,
        Search,
        DataTableSkeleton,
        InlineNotification,
        Modal,
        TextInput,
        Select,
        SelectItem,
        Pagination,
    } from "carbon-components-svelte";
    import { Filter, Add, Edit, TrashCan, Close } from "carbon-icons-svelte";
    import { fade } from "svelte/transition";
    import ItemForm from "../components/ItemForm.svelte";
    import { onMount, onDestroy } from "svelte";
    import {
        fetchNFeIdentifications,
        createNFeIdentification,
        updateNFeIdentification,
        deleteNFeIdentification,
    } from "../services/api";
    import type { NFeIdentification } from "../types/nfe";

    export const navigateTo = (path: string) => {
        if (typeof window !== "undefined") {
            window.history.pushState({}, "", path);
        }
    };

    let searchQuery = "";
    let isFilterPanelOpen = false;
    let editingItem: NFeIdentification | null = null;
    let isFormModalOpen = false;
    let isDeleteModalOpen = false;
    let itemToDelete: string | null = null;
    let errorMessage: string | null = null;
    let loading = false;
    let currentPage = 1;
    let totalCount = 0;
    let hasMore = false;
    let items: NFeIdentification[] = [];
    let visibleItems: NFeIdentification[] = [];

    // Filter state
    let filters = {
        natOp: "",
        nNF: "",
        tpNF: "",
        dhEmi: "",
    };

    // Pagination state
    let pageSize = 50;
    let pageSizes = [25, 50, 100, 200, 500];

    interface TableRow {
        id: string;
        internal_key: string;
        natOp: string;
        nNF: string;
        dhEmi: string;
        tpNF: string;
    }

    interface TableHeader {
        key: keyof TableRow | "actions";
        label: string;
    }

    const headers: TableHeader[] = [
        { key: "internal_key", label: "Internal Key" },
        { key: "natOp", label: "Natureza da Operação" },
        { key: "nNF", label: "Número NF" },
        { key: "dhEmi", label: "Data Emissão" },
        { key: "tpNF", label: "Tipo NF" },
        { key: "actions", label: "Ações" },
    ];

    // Memory efficient filtering - only filter visible items
    function applyFilters(
        itemsToFilter: NFeIdentification[],
    ): NFeIdentification[] {
        return itemsToFilter.filter((item) => {
            // Apply search query across all fields
            if (searchQuery) {
                const query = searchQuery.toLowerCase();
                const matchesSearch =
                    (item.internal_key?.toLowerCase() || "").includes(query) ||
                    (item.natOp?.toLowerCase() || "").includes(query) ||
                    (item.nNF || "").includes(query) ||
                    (item.dhEmi || "").includes(query) ||
                    (item.tpNF || "").includes(query);

                if (!matchesSearch) return false;
            }

            // Apply specific filters
            const matchesNatOp =
                !filters.natOp ||
                (item.natOp?.toLowerCase() || "").includes(
                    filters.natOp.toLowerCase(),
                );
            const matchesNNF =
                !filters.nNF || (item.nNF || "").includes(filters.nNF);
            const matchesTpNF = !filters.tpNF || item.tpNF === filters.tpNF;
            const matchesDhEmi =
                !filters.dhEmi || (item.dhEmi || "").includes(filters.dhEmi);

            return matchesNatOp && matchesNNF && matchesTpNF && matchesDhEmi;
        });
    }

    // Reactive declarations moved to top level
    $: filteredItems = applyFilters(visibleItems);
    $: rows = filteredItems.map(
        (item) =>
            ({
                id: item.internal_key,
                internal_key: item.internal_key,
                natOp: item.natOp || "",
                nNF: item.nNF || "",
                dhEmi: item.dhEmi
                    ? new Date(item.dhEmi).toLocaleDateString()
                    : "",
                tpNF: item.tpNF === "1" ? "Output" : "Input",
            }) as TableRow,
    );

    function clearFilters() {
        filters = {
            natOp: "",
            nNF: "",
            tpNF: "",
            dhEmi: "",
        };
        searchQuery = "";
    }

    function handlePaginationChange(e: CustomEvent) {
        const { pageSize: newPageSize, page: newPage } = e.detail;
        console.log(
            `[UI] Pagination change: page ${newPage}, pageSize ${newPageSize}`,
        );

        // Ensure pageSize is always defined
        const effectivePageSize = newPageSize || pageSize;
        const effectivePage = newPage || 1;

        // Reset to page 1 when changing page size
        if (effectivePageSize !== pageSize) {
            console.log(
                `[UI] Page size changed from ${pageSize} to ${effectivePageSize}, resetting to page 1`,
            );
            // Update pageSize first
            pageSize = effectivePageSize;
            // Then reset other state
            currentPage = 1;
            items = [];
            visibleItems = [];
            // Force a state update
            refreshItems();
        } else if (effectivePage !== currentPage) {
            console.log(
                `[UI] Page changed from ${currentPage} to ${effectivePage}`,
            );
            currentPage = effectivePage;
            refreshItems();
        }
    }

    async function refreshItems() {
        try {
            console.log(
                `[UI] Refreshing items for page ${currentPage} with pageSize ${pageSize}`,
            );
            loading = true;
            const response = await fetchNFeIdentifications({
                page: currentPage,
                pageSize,
            });

            console.log(
                `[UI] Received response with ${response.data.length} items`,
            );

            // Clear previous items if on page 1
            if (currentPage === 1) {
                console.log("[UI] Clearing previous items (page 1)");
                items = [];
            }

            // Append new items
            items = [...items, ...response.data];

            // Update visible items immediately
            const start = (currentPage - 1) * pageSize;
            const end = start + pageSize;
            visibleItems = items.slice(start, end);
            console.log(
                `[UI] Updated visible items: showing items ${start} to ${end} of ${items.length}`,
            );

            totalCount = response.total;
            hasMore = currentPage < Math.ceil(totalCount / pageSize);
            errorMessage = null;
            console.log(
                `[UI] Updated state - total: ${totalCount}, hasMore: ${hasMore}`,
            );
        } catch (error) {
            console.error("[UI] Error refreshing items:", error);
            errorMessage =
                error instanceof Error ? error.message : "Failed to load items";
        } finally {
            loading = false;
            console.log("[UI] Loading complete");
        }
    }

    onMount(() => {
        let interval: number;
        console.log("[UI] Component mounted");

        // Initialize data
        refreshItems().catch((error) => {
            console.error("[UI] Initial data load failed:", error);
            errorMessage =
                error instanceof Error ? error.message : "Failed to load items";
        });

        // Add memory optimization cleanup
        interval = window.setInterval(() => {
            // Free memory for items not in the visible window
            if (items.length > 1000) {
                console.log(
                    `[UI] Memory cleanup: reducing items array size to 1000`,
                );
                // Keep only the most recent items
                const startIndex = Math.max(0, items.length - 1000);
                items = items.slice(startIndex);
                // Update visible items based on current page
                const start = (currentPage - 1) * pageSize;
                const end = start + pageSize;
                visibleItems = items.slice(start, end);
            }
        }, 30000); // Check every 30 seconds

        return () => {
            console.log("[UI] Component cleanup");
            clearInterval(interval);
        };
    });

    onDestroy(() => {
        // Clear large data structures on component destruction
        items = [];
        visibleItems = [];
    });

    async function handleCreate(
        event: CustomEvent<Omit<NFeIdentification, "internal_key">>,
    ): Promise<void> {
        try {
            // Create a new item with a temporary internal_key
            const newItem = await createNFeIdentification({
                ...event.detail,
                internal_key: `temp_${Date.now()}`,
            });
            items = [newItem, ...items];
            visibleItems = items.slice(0, pageSize);
            isFormModalOpen = false;
        } catch (error) {
            errorMessage =
                error instanceof Error
                    ? error.message
                    : "Failed to create item";
        }
    }

    async function handleUpdate(
        event: CustomEvent<Omit<NFeIdentification, "internal_key">>,
    ): Promise<void> {
        if (!editingItem?.id) return;
        try {
            const updatedItem = await updateNFeIdentification(editingItem.id, {
                ...editingItem,
                ...event.detail,
            });

            // Only update the item in the arrays without creating new arrays
            const itemIndex = items.findIndex(
                (item) => item.id === updatedItem.id,
            );
            if (itemIndex >= 0) {
                items[itemIndex] = updatedItem;
            }

            const visibleIndex = visibleItems.findIndex(
                (item) => item.id === updatedItem.id,
            );
            if (visibleIndex >= 0) {
                visibleItems[visibleIndex] = updatedItem;
            }

            // Force svelte to recognize the change
            items = [...items];
            visibleItems = [...visibleItems];

            isFormModalOpen = false;
            editingItem = null;
        } catch (error) {
            errorMessage =
                error instanceof Error
                    ? error.message
                    : "Failed to update item";
        }
    }

    async function handleDelete(): Promise<void> {
        if (!itemToDelete) return;
        try {
            const item = items.find((i) => i.internal_key === itemToDelete);
            if (!item?.id) return;

            await deleteNFeIdentification(item.id);

            // Remove from both arrays
            items = items.filter((item) => item.internal_key !== itemToDelete);
            visibleItems = visibleItems.filter(
                (item) => item.internal_key !== itemToDelete,
            );

            isDeleteModalOpen = false;
            itemToDelete = null;
        } catch (error) {
            errorMessage =
                error instanceof Error
                    ? error.message
                    : "Failed to delete item";
        }
    }

    function handleEdit(row: TableRow): void {
        const item = items.find((i) => i.internal_key === row.id);
        if (item) {
            editingItem = { ...item }; // Create a copy to avoid reference issues
            isFormModalOpen = true;
        }
    }

    function handleDeleteClick(row: TableRow): void {
        itemToDelete = row.id;
        isDeleteModalOpen = true;
    }

    let selectedRow: TableRow | null = null;

    function handleRowClick(row: TableRow) {
        selectedRow = row;
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
                    <Select labelText="Type" bind:selected={filters.tpNF}>
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

    {#if errorMessage}
        <InlineNotification
            kind="error"
            title="Error"
            subtitle={errorMessage}
            on:close={() => (errorMessage = null)}
        />
    {/if}

    <div class="table-summary">
        <span class="items-count"
            >{totalCount} total items (showing {filteredItems.length} items on page
            {currentPage} of {Math.ceil(totalCount / pageSize)})</span
        >
    </div>

    {#if loading && currentPage === 1}
        <DataTableSkeleton />
    {:else}
        <div class="table-container">
            <table class="data-table">
                <thead>
                    <tr>
                        {#each headers as { key, label }}
                            <th class="header-{key}">{label}</th>
                        {/each}
                    </tr>
                </thead>
                <tbody>
                    {#each rows as row (row.id)}
                        <tr
                            class:selected={selectedRow?.id === row.id}
                            on:click={() => handleRowClick(row)}
                        >
                            {#each headers as { key }}
                                <td
                                    class="cell-{key}"
                                    data-column={key}
                                    aria-label={key}
                                >
                                    {#if key === "actions"}
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
                                                on:click={() =>
                                                    handleDeleteClick(row)}
                                            >
                                                Delete
                                            </Button>
                                        </div>
                                    {:else}
                                        {row[key as keyof TableRow]}
                                    {/if}
                                </td>
                            {/each}
                        </tr>
                    {/each}
                </tbody>
            </table>

            <div class="pagination-container">
                <Pagination
                    page={currentPage}
                    {pageSize}
                    {pageSizes}
                    totalItems={totalCount}
                    on:change={handlePaginationChange}
                />
            </div>
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
        background: #121212;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }

    /* Page Layout */
    .page-container {
        padding: 24px;
        min-height: 100vh;
        background: #121212;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin: -24px -24px 32px -24px;
        padding: 24px;
        background: #004431;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
    }

    h1 {
        font-size: 24px;
        font-weight: 600;
        color: #ffd700;
        margin: 0;
        letter-spacing: -0.025em;
    }

    .header-actions {
        display: flex;
        gap: 12px;
        align-items: center;
    }

    /* Table container */
    .table-container {
        margin: 1rem 0;
        position: relative;
        background: #1e1e1e;
        border-radius: 8px;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
    }

    /* Table styles */
    .data-table {
        width: 100%;
        border-collapse: collapse;
        margin-top: 1rem;
        background-color: #1e1e1e;
    }

    .data-table th {
        background-color: #004431;
        color: #ffd700;
        font-weight: 600;
        padding: 0.75rem 1rem;
        text-align: left;
        border-bottom: 1px solid #ffd700;
    }

    .data-table td {
        padding: 0.75rem 1rem;
        border-bottom: 1px solid #333333;
        color: #ffffff;
    }

    .data-table tr {
        transition: background-color 0.2s ease;
    }

    .data-table tr:hover {
        background-color: #333333;
        cursor: pointer;
    }

    .data-table tr.selected {
        background-color: #ffd700;
        color: #121212;
    }

    /* Pagination styles */
    .pagination-container {
        margin-top: 1rem;
        padding: 1rem;
        background: #1e1e1e;
        border-top: 1px solid #ffd700;
    }

    /* Filter panel */
    .filter-panel {
        background: #1e1e1e;
        border-radius: 8px;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
        margin-bottom: 1rem;
        padding: 1rem;
    }

    .filter-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
    }

    .filter-content {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1rem;
    }

    .filter-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    /* Actions */
    .actions {
        display: flex;
        gap: 0.5rem;
    }

    /* Table summary */
    .table-summary {
        margin: 1rem 0;
        color: #ffd700;
        font-size: 0.875rem;
    }

    .data-table th.header-actions {
        text-align: right;
    }

    .data-table td[data-column="actions"] {
        text-align: right;
    }
</style>
