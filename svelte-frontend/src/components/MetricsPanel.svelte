<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import {
        metricsService,
        type OperationMetric,
    } from "../services/metricsService";
    import {
        Modal,
        DataTable,
        Button,
        Tabs,
        Tab,
        TabContent,
        StructuredList,
        StructuredListHead,
        StructuredListRow,
        StructuredListCell,
        StructuredListBody,
        InlineNotification,
        Loading,
    } from "carbon-components-svelte";
    import { Download, Reset } from "carbon-icons-svelte";

    export let open = false;

    let metrics: OperationMetric[] = [];
    let operationCounts: Record<string, number> = {};
    let averageDurations: Record<string, number | null> = {};
    let refreshInterval: number;
    let errorMessage: string | null = null;
    let activeTab = "Summary";
    let currentPage = 1;
    let totalCount = 0;
    let hasMore = false;
    let isLoading = false;
    let showLoadMorePrompt = false;

    // Set up headers for the operations table
    const headers = [
        { key: "type", value: "Operation Type", empty: false },
        { key: "timestamp", value: "Timestamp", empty: false },
        { key: "duration", value: "Duration (ms)", empty: false },
        { key: "details", value: "Details", empty: false },
    ];

    // Memoize the rows transformation
    let rowsCache: any[] = [];
    let lastMetricsLength = 0;

    function updateRowsCache() {
        if (metrics.length !== lastMetricsLength) {
            rowsCache = metrics.map((metric, index) => ({
                id: String(index),
                type: metric.type || "Unknown",
                timestamp: metric.timestamp
                    ? new Date(metric.timestamp).toLocaleString()
                    : "N/A",
                duration:
                    metric.duration != null ? metric.duration.toFixed(2) : "-",
                details: formatDetails(metric.details),
            }));
            lastMetricsLength = metrics.length;
        }
    }

    // Format details safely
    function formatDetails(details: any): string {
        if (!details) return "-";
        try {
            return typeof details === "object"
                ? JSON.stringify(details, null, 2)
                : String(details);
        } catch (e) {
            return "Error displaying details";
        }
    }

    // Get operation types for tabs
    $: operationTypes = Object.keys(operationCounts).sort();

    async function refreshMetrics() {
        try {
            isLoading = true;
            const response =
                await metricsService.getMetricsPaginated(currentPage);
            if (JSON.stringify(response.data) !== JSON.stringify(metrics)) {
                metrics = response.data;
                totalCount = response.totalCount;
                hasMore = response.hasMore;
                operationCounts = metricsService.getOperationCounts();
                updateRowsCache();

                // Only calculate averages for the active tab
                if (activeTab !== "Summary" && activeTab !== "All Operations") {
                    averageDurations[activeTab] =
                        metricsService.getAverageDuration(activeTab);
                } else {
                    operationTypes.forEach((type) => {
                        averageDurations[type] =
                            metricsService.getAverageDuration(type);
                    });
                }
            }
            errorMessage = null;
        } catch (error) {
            errorMessage =
                "Error refreshing metrics: " +
                (error instanceof Error ? error.message : "Unknown error");
        } finally {
            isLoading = false;
        }
    }

    async function loadMoreMetrics() {
        if (hasMore) {
            currentPage++;
            await refreshMetrics();
        }
    }

    function handleScroll(
        e: CustomEvent<{
            scrollTop: number;
            scrollHeight: number;
            clientHeight: number;
        }>,
    ) {
        const { scrollTop, scrollHeight, clientHeight } = e.detail;
        if (
            scrollHeight - scrollTop <= clientHeight * 1.5 &&
            !isLoading &&
            hasMore
        ) {
            showLoadMorePrompt = true;
        }
    }

    function clearMetrics() {
        try {
            metricsService.clearMetrics();
            metrics = [];
            operationCounts = {};
            averageDurations = {};
            rowsCache = [];
            lastMetricsLength = 0;
            errorMessage = null;
        } catch (error) {
            errorMessage =
                "Error clearing metrics: " +
                (error instanceof Error ? error.message : "Unknown error");
        }
    }

    function downloadMetrics() {
        try {
            const dataStr =
                "data:text/json;charset=utf-8," +
                encodeURIComponent(JSON.stringify(metrics, null, 2));
            const downloadAnchorNode = document.createElement("a");
            downloadAnchorNode.setAttribute("href", dataStr);
            downloadAnchorNode.setAttribute(
                "download",
                `metrics-${new Date().toISOString()}.json`,
            );
            document.body.appendChild(downloadAnchorNode);
            downloadAnchorNode.click();
            downloadAnchorNode.remove();
            errorMessage = null;
        } catch (error) {
            errorMessage =
                "Error downloading metrics: " +
                (error instanceof Error ? error.message : "Unknown error");
        }
    }

    onMount(() => {
        refreshMetrics();
        refreshInterval = window.setInterval(() => {
            if (open) {
                refreshMetrics();
            }
        }, 30000);
    });

    onDestroy(() => {
        clearInterval(refreshInterval);
    });

    // Only refresh when the panel opens
    $: if (open) {
        refreshMetrics();
    }
</script>

<Modal
    bind:open
    modalHeading="Operations Metrics"
    primaryButtonText="Close"
    secondaryButtons={[{ text: "Clear Metrics" }, { text: "Download" }]}
    size="lg"
    passiveModal
    on:click:button--secondary={(e) => {
        if (e.detail.text === "Clear Metrics") {
            clearMetrics();
        } else if (e.detail.text === "Download") {
            downloadMetrics();
        }
    }}
>
    {#if errorMessage}
        <InlineNotification
            kind="error"
            title="Error"
            subtitle={errorMessage}
            on:close={() => (errorMessage = null)}
        />
    {/if}

    {#if isLoading}
        <Loading active />
    {/if}

    <Tabs bind:selected={activeTab}>
        <Tab label="Summary" />
        <Tab label="All Operations" />
        {#each operationTypes as type}
            <Tab label={type} />
        {/each}

        <svelte:fragment slot="content">
            <TabContent>
                <div class="metrics-summary">
                    <h3>Operation Counts</h3>
                    <StructuredList>
                        <StructuredListHead>
                            <StructuredListRow head>
                                <StructuredListCell head
                                    >Operation Type</StructuredListCell
                                >
                                <StructuredListCell head
                                    >Count</StructuredListCell
                                >
                                <StructuredListCell head
                                    >Avg. Duration (ms)</StructuredListCell
                                >
                            </StructuredListRow>
                        </StructuredListHead>
                        <StructuredListBody>
                            {#each operationTypes as type}
                                <StructuredListRow>
                                    <StructuredListCell
                                        >{type}</StructuredListCell
                                    >
                                    <StructuredListCell
                                        >{operationCounts[
                                            type
                                        ]}</StructuredListCell
                                    >
                                    <StructuredListCell>
                                        {averageDurations[type]
                                            ? averageDurations[type].toFixed(2)
                                            : "-"}
                                    </StructuredListCell>
                                </StructuredListRow>
                            {/each}
                        </StructuredListBody>
                    </StructuredList>
                </div>
            </TabContent>

            <TabContent>
                <DataTable {headers} rows={rowsCache} sortable />
            </TabContent>

            {#each operationTypes as type}
                <TabContent>
                    <DataTable
                        {headers}
                        rows={rowsCache.filter((row) => row.type === type)}
                        sortable
                    />
                </TabContent>
            {/each}
        </svelte:fragment>
    </Tabs>

    {#if showLoadMorePrompt && hasMore}
        <div class="load-more-prompt">
            <Button
                kind="tertiary"
                on:click={() => {
                    showLoadMorePrompt = false;
                    loadMoreMetrics();
                }}
            >
                Load More ({totalCount - metrics.length} remaining)
            </Button>
        </div>
    {/if}
</Modal>

<style>
    .metrics-summary {
        padding: 1rem 0;
    }

    h3 {
        margin-bottom: 1rem;
        color: #32cd32;
    }

    :global(.bx--structured-list) {
        margin-bottom: 2rem;
    }

    :global(.bx--structured-list-thead .bx--structured-list-row) {
        border-bottom: 2px solid #32cd32;
    }

    :global(.bx--structured-list-tbody .bx--structured-list-row) {
        border-bottom: 1px solid #32cd32;
    }

    :global(.bx--structured-list-cell) {
        color: #90ee90;
    }

    :global(
            .bx--structured-list-row.bx--structured-list-row--header-row
                .bx--structured-list-cell
        ) {
        color: #32cd32;
        font-weight: 600;
    }

    .load-more-prompt {
        display: flex;
        justify-content: center;
        padding: 1rem;
        margin-top: 1rem;
        border-top: 1px solid #32cd32;
    }

    :global(.bx--data-table-container) {
        max-height: 60vh;
        overflow-y: auto;
    }
</style>
