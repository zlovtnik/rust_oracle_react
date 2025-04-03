export interface OperationMetric {
    type: string;
    timestamp: Date;
    duration?: number;
    details?: Record<string, any>;
}

interface PaginatedResponse<T> {
    data: T[];
    totalCount: number;
    hasMore: boolean;
}

class MetricsService {
    private metrics: OperationMetric[] = [];
    private operationStartTimes: Map<string, number> = new Map();
    private readonly STORAGE_KEY = 'nfe_metrics';
    private saveTimeout: number | null = null;
    private readonly SAVE_DELAY = 1000; // 1 second delay before saving
    private readonly MAX_METRICS = 1000; // Maximum number of metrics to store
    private readonly MAX_AGE = 24 * 60 * 60 * 1000; // 24 hours in milliseconds
    private readonly PAGE_SIZE = 999; // Number of items per page

    constructor() {
        this.loadMetrics();
    }

    private loadMetrics() {
        try {
            const storedMetrics = localStorage.getItem(this.STORAGE_KEY);
            if (storedMetrics) {
                const parsed = JSON.parse(storedMetrics);
                this.metrics = parsed.map((m: any) => ({
                    ...m,
                    timestamp: new Date(m.timestamp)
                }));
                this.cleanupOldMetrics();
            }
        } catch (error) {
            console.error('Error loading metrics:', error);
        }
    }

    private cleanupOldMetrics() {
        const now = Date.now();
        this.metrics = this.metrics
            .filter(metric => now - metric.timestamp.getTime() < this.MAX_AGE)
            .slice(-this.MAX_METRICS);
    }

    private debouncedSave() {
        if (this.saveTimeout) {
            window.clearTimeout(this.saveTimeout);
        }
        this.saveTimeout = window.setTimeout(() => {
            try {
                this.cleanupOldMetrics();
                localStorage.setItem(this.STORAGE_KEY, JSON.stringify(this.metrics));
            } catch (error) {
                console.error('Error saving metrics:', error);
            }
            this.saveTimeout = null;
        }, this.SAVE_DELAY);
    }

    // Record the start of an operation for duration tracking
    startOperation(operationId: string): void {
        this.operationStartTimes.set(operationId, performance.now());
    }

    // Record a completed operation with its duration
    recordOperation(type: string, details?: Record<string, any>, operationId?: string): void {
        let duration: number | undefined;

        if (operationId && this.operationStartTimes.has(operationId)) {
            const startTime = this.operationStartTimes.get(operationId);
            duration = performance.now() - startTime!;
            this.operationStartTimes.delete(operationId);
        }

        this.metrics.push({
            type,
            timestamp: new Date(),
            duration,
            details
        });

        // Debounce the save operation
        this.debouncedSave();

        // Log to console in development mode only
        const isDev = import.meta.env?.MODE !== 'production';
        if (isDev) {
            console.log(`Metric recorded: ${type}`, { duration, details });
        }
    }

    // Get all metrics
    getMetrics(): OperationMetric[] {
        this.cleanupOldMetrics();
        return [...this.metrics];
    }

    // Get metrics of a specific type
    getMetricsByType(type: string): OperationMetric[] {
        this.cleanupOldMetrics();
        return this.metrics.filter(metric => metric.type === type);
    }

    // Get average duration for a type of operation
    getAverageDuration(type: string): number | null {
        const typeMetrics = this.getMetricsByType(type).filter(m => m.duration !== undefined);
        if (typeMetrics.length === 0) return null;

        const totalDuration = typeMetrics.reduce((sum, metric) => sum + (metric.duration || 0), 0);
        return totalDuration / typeMetrics.length;
    }

    // Get operation counts by type
    getOperationCounts(): Record<string, number> {
        this.cleanupOldMetrics();
        return this.metrics.reduce((counts, metric) => {
            counts[metric.type] = (counts[metric.type] || 0) + 1;
            return counts;
        }, {} as Record<string, number>);
    }

    // Clear all metrics
    clearMetrics(): void {
        this.metrics = [];
        this.operationStartTimes.clear();
        this.debouncedSave();
    }

    getMetricsPaginated(page: number = 1): PaginatedResponse<OperationMetric> {
        this.cleanupOldMetrics();
        const startIndex = (page - 1) * this.PAGE_SIZE;
        const endIndex = startIndex + this.PAGE_SIZE;
        const paginatedData = this.metrics.slice(startIndex, endIndex);

        return {
            data: paginatedData,
            totalCount: this.metrics.length,
            hasMore: endIndex < this.metrics.length
        };
    }
}

// Export a singleton instance
export const metricsService = new MetricsService();
