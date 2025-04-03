<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { metricsService } from "../services/metricsService";
    import {
        Modal,
        DataTable,
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
    import { ChartBar } from "carbon-icons-svelte";
    import type { MetricsData } from "../types/metrics";

    export let open = false;
    export let metrics: MetricsData;
    export let isLoading = false;
    export let error: string | null = null;

    let operationCounts: Record<string, number> = {};
    let refreshInterval: number;
    let activeTab = 0;

    interface ChartData {
        labels: string[];
        values: number[];
    }

    interface TableRow {
        id: string;
        type: string;
        timestamp: string;
        duration: string;
        details: string;
    }

    // Set up headers for the operations table
    const headers = [
        { key: "type", value: "Operation Type", empty: false },
        { key: "timestamp", value: "Timestamp", empty: false },
        { key: "duration", value: "Duration (ms)", empty: false },
        { key: "details", value: "Details", empty: false },
    ];

    let rowsCache: TableRow[] = [];

    function updateRowsCache() {
        if (metrics.operationTypes) {
            rowsCache = Object.entries(metrics.operationTypes).map(
                ([type, count], index) => ({
                    id: String(index),
                    type,
                    timestamp: new Date().toLocaleString(),
                    duration: metrics.averageDuration.toFixed(2),
                    details: `Count: ${count}`,
                }),
            );
        }
    }

    function formatChartData(data: MetricsData): ChartData {
        return {
            labels: Object.keys(data.operationTypes),
            values: Object.values(data.operationTypes),
        };
    }

    $: chartData = formatChartData(metrics);

    async function refreshMetrics() {
        try {
            isLoading = true;
            const data = await metricsService.getMetrics();
            metrics = data;
            operationCounts = data.operationTypes;
            updateRowsCache();
            error = null;
        } catch (err) {
            error =
                "Error refreshing metrics: " +
                (err instanceof Error ? err.message : "Unknown error");
        } finally {
            isLoading = false;
        }
    }

    function clearMetrics() {
        try {
            metricsService.clearCache();
            metrics = {
                totalOperations: 0,
                averageDuration: 0,
                operationTypes: {},
            };
            operationCounts = {};
            rowsCache = [];
            error = null;
        } catch (err) {
            error =
                "Error clearing metrics: " +
                (err instanceof Error ? err.message : "Unknown error");
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
    secondaryButtons={[{ text: "Clear Metrics" }, { text: "Close" }]}
    size="lg"
    passiveModal
    on:click:button--secondary={(e) => {
        if (e.detail.text === "Clear Metrics") {
            clearMetrics();
        }
    }}
>
    {#if error}
        <InlineNotification
            kind="error"
            title="Error"
            subtitle={error}
            on:close={() => (error = null)}
        />
    {/if}

    {#if isLoading}
        <Loading active />
    {/if}

    <Tabs bind:selected={activeTab}>
        <Tab label="Summary" />
        <Tab label="All Operations" />

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
                            {#each Object.entries(operationCounts) as [type, count]}
                                <StructuredListRow>
                                    <StructuredListCell
                                        >{type}</StructuredListCell
                                    >
                                    <StructuredListCell
                                        >{count}</StructuredListCell
                                    >
                                    <StructuredListCell>
                                        {metrics.averageDuration.toFixed(2)}
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
        </svelte:fragment>
    </Tabs>
</Modal>

<div class="metrics-panel">
    {#if error}
        <div class="error-message">{error}</div>
    {:else if isLoading}
        <div class="loading-indicator">Loading metrics...</div>
    {:else}
        <div class="chart-container">
            <ChartBar class="chart-icon" />
            <div class="chart-data">
                {#each chartData.labels as label, i}
                    <div class="metric-item">
                        <span class="metric-label">{label}:</span>
                        <span class="metric-value">{chartData.values[i]}</span>
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</div>

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

    .metrics-panel {
        background-color: #1e1e1e;
        border-radius: 4px;
        padding: 1rem;
        margin: 1rem 0;
    }

    .chart-container {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .chart-data {
        flex: 1;
    }

    .metric-item {
        display: flex;
        justify-content: space-between;
        margin: 0.5rem 0;
    }

    .metric-label {
        color: #ffd700;
    }

    .metric-value {
        color: #ffffff;
    }

    .error-message {
        color: #ff6b6b;
        text-align: center;
    }

    .loading-indicator {
        color: #ffd700;
        text-align: center;
    }
</style>
