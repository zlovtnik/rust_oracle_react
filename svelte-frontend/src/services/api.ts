import type { NFeIdentification } from '../types/nfe';
import { metricsService } from './metricsService';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';
const CACHE_DURATION = 5 * 60 * 1000; // 5 minutes
const BATCH_DELAY = 100; // 100ms delay for batching

interface CacheEntry<T> {
    data: T;
    timestamp: number;
    totalCount: number;
}

class ApiCache {
    private cache: Map<string, CacheEntry<unknown>> = new Map();
    private pendingUpdates: Set<string> = new Set();
    private updateTimeout: number | null = null;

    get<T>(key: string): CacheEntry<T> | null {
        const entry = this.cache.get(key);
        if (!entry) return null;

        if (Date.now() - entry.timestamp > CACHE_DURATION) {
            this.cache.delete(key);
            return null;
        }

        return entry as CacheEntry<T>;
    }

    set<T>(key: string, data: T, totalCount: number): void {
        this.cache.set(key, {
            data,
            timestamp: Date.now(),
            totalCount
        });
    }

    markForUpdate(key: string): void {
        this.pendingUpdates.add(key);
        if (!this.updateTimeout) {
            this.updateTimeout = window.setTimeout(() => {
                this.pendingUpdates.forEach(key => this.cache.delete(key));
                this.pendingUpdates.clear();
                this.updateTimeout = null;
            }, BATCH_DELAY);
        }
    }

    clear(): void {
        this.cache.clear();
    }
}

const apiCache = new ApiCache();

// Debounce function for API calls
function debounce<T, Args extends unknown[]>(fn: (...args: Args) => Promise<T>, delay: number) {
    let timeoutId: number | null = null;
    return async (...args: Args): Promise<T> => {
        if (timeoutId) {
            window.clearTimeout(timeoutId);
        }
        return new Promise((resolve) => {
            timeoutId = window.setTimeout(async () => {
                const result = await fn(...args);
                resolve(result);
            }, delay);
        });
    };
}

export interface FilterParams {
    natOp?: string;
    nNF?: string;
    tpNF?: string;
    dhEmi?: string;
    searchQuery?: string;
}

export interface QueryParams {
    page?: number;
    pageSize?: number;
    sortBy?: string;
    sortOrder?: 'asc' | 'desc';
    filters?: Record<string, string>;
}

interface ApiResponse<T> {
    data: T;
    total: number;
    page: number;
    pageSize: number;
}

export async function fetchNFeIdentifications(params: QueryParams = {}): Promise<ApiResponse<NFeIdentification[]>> {
    const queryString = new URLSearchParams();

    if (params.page) queryString.append('page', params.page.toString());
    if (params.pageSize) queryString.append('pageSize', params.pageSize.toString());
    if (params.sortBy) queryString.append('sortBy', params.sortBy);
    if (params.sortOrder) queryString.append('sortOrder', params.sortOrder);
    if (params.filters) {
        Object.entries(params.filters).forEach(([key, value]) => {
            queryString.append(key, value);
        });
    }

    const response = await fetch(`${API_BASE_URL}/api/identifications?${queryString}`);
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
}

export async function createNFeIdentification(data: NFeIdentification): Promise<NFeIdentification> {
    const response = await fetch(`${API_BASE_URL}/api/identifications`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    });
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
}

export async function updateNFeIdentification(id: number, data: NFeIdentification): Promise<NFeIdentification> {
    const response = await fetch(`${API_BASE_URL}/api/identifications/${id}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    });
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
}

export async function deleteNFeIdentification(id: number): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/api/identifications/${id}`, {
        method: 'DELETE',
    });
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }
}

export const createIdentification = debounce(async (data: Partial<NFeIdentification>): Promise<NFeIdentification> => {
    const operationId = `create_identification_${Date.now()}`;
    metricsService.startOperation(operationId);
    try {
        const response = await fetch(`${API_BASE_URL}/identifications`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        });
        if (!response.ok) {
            throw new Error('Failed to create identification');
        }
        const result = await response.json();
        apiCache.markForUpdate('identifications');
        metricsService.recordOperation("create_identification", { success: true }, operationId);
        return result;
    } catch (error: unknown) {
        const errorMessage = error instanceof Error ? error.message : 'Unknown error';
        metricsService.recordOperation("create_identification_error", { error: errorMessage }, operationId);
        throw error;
    }
}, BATCH_DELAY);

export const deleteIdentification = debounce(async (id: string): Promise<void> => {
    const operationId = `delete_identification_${Date.now()}`;
    metricsService.startOperation(operationId);
    try {
        const response = await fetch(`${API_BASE_URL}/identifications/${id}`, {
            method: 'DELETE',
        });
        if (!response.ok) {
            throw new Error('Failed to delete identification');
        }
        apiCache.markForUpdate('identifications');
        metricsService.recordOperation("delete_identification", { success: true }, operationId);
    } catch (error: unknown) {
        const errorMessage = error instanceof Error ? error.message : 'Unknown error';
        metricsService.recordOperation("delete_identification_error", { error: errorMessage }, operationId);
        throw error;
    }
}, BATCH_DELAY); 