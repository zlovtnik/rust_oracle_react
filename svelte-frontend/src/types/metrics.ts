export interface MetricsData {
    totalOperations: number;
    averageDuration: number;
    operationTypes: Record<string, number>;
} 