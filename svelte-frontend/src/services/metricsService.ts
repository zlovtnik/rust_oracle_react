import type { MetricsData } from '../types/metrics';

interface OperationMetric {
    id: number;
    operationType: string;
    duration: number;
    timestamp: string;
}

interface MetricsCache {
    data: OperationMetric[];
    timestamp: number;
}

class MetricsService {
    private cache: MetricsCache = {
        data: [],
        timestamp: 0,
    };

    private operationStartTimes: Map<string, number> = new Map();
    private readonly CACHE_DURATION = 5 * 60 * 1000; // 5 minutes

    async getMetrics(): Promise<MetricsData> {
        const now = Date.now();
        if (now - this.cache.timestamp < this.CACHE_DURATION) {
            return this.processMetrics(this.cache.data);
        }

        try {
            const response = await fetch('/api/metrics');
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data: OperationMetric[] = await response.json();

            this.cache = {
                data,
                timestamp: now,
            };

            return this.processMetrics(data);
        } catch (error) {
            console.error('Error fetching metrics:', error);
            throw error;
        }
    }

    private processMetrics(data: OperationMetric[]): MetricsData {
        const metrics: MetricsData = {
            totalOperations: data.length,
            averageDuration: 0,
            operationTypes: {},
        };

        if (data.length === 0) {
            return metrics;
        }

        let totalDuration = 0;
        data.forEach((metric) => {
            totalDuration += metric.duration;
            metrics.operationTypes[metric.operationType] = (metrics.operationTypes[metric.operationType] || 0) + 1;
        });

        metrics.averageDuration = totalDuration / data.length;
        return metrics;
    }

    startOperation(operationId: string): void {
        this.operationStartTimes.set(operationId, performance.now());
    }

    recordOperation(type: string, details?: Record<string, unknown>, operationId?: string): void {
        let duration: number | undefined;

        if (operationId && this.operationStartTimes.has(operationId)) {
            const startTime = this.operationStartTimes.get(operationId);
            duration = performance.now() - startTime!;
            this.operationStartTimes.delete(operationId);
        }

        const metric: OperationMetric = {
            id: Date.now(),
            operationType: type,
            duration: duration || 0,
            timestamp: new Date().toISOString(),
        };

        this.cache.data.push(metric);
        this.cache.timestamp = Date.now();
    }

    clearCache(): void {
        this.cache = {
            data: [],
            timestamp: 0,
        };
        this.operationStartTimes.clear();
    }
}

export const metricsService = new MetricsService();
