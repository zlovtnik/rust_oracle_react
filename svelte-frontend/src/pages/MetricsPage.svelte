<script lang="ts">
    import MetricsPanel from "../components/MetricsPanel.svelte";
    import { Button } from "carbon-components-svelte";
    import { ChartBar } from "carbon-icons-svelte";
    import { metricsService } from "../services/metricsService";
    import { onMount } from "svelte";

    let isMetricsOpen = false;
    let hasMetrics = false;

    onMount(() => {
        // Check if we have any metrics
        hasMetrics = metricsService.getMetrics().length > 0;
    });

    // Watch for changes in metrics
    $: hasMetrics = metricsService.getMetrics().length > 0;
</script>

<div class="metrics-page">
    <div class="page-header">
        <h1>System Metrics</h1>
        <div class="header-actions">
            {#if !hasMetrics}
                <div class="no-metrics-message">
                    No metrics recorded yet. Perform some operations to see
                    metrics.
                </div>
            {/if}
            <Button
                kind="primary"
                icon={ChartBar}
                on:click={() => (isMetricsOpen = true)}
                disabled={!hasMetrics}
            >
                View Metrics
            </Button>
        </div>
    </div>

    <MetricsPanel bind:open={isMetricsOpen} />
</div>

<style>
    .metrics-page {
        padding: 2rem;
    }

    .page-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }

    .header-actions {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .no-metrics-message {
        color: #90ee90;
        background: #4b0082;
        padding: 0.5rem 1rem;
        border-radius: 4px;
        border: 1px solid #32cd32;
    }

    h1 {
        color: var(--white);
        margin: 0;
    }
</style>
