import type { NFeIdentification } from '../types/nfeTypes';
import { metricsService } from './metricsService';

const API_BASE_URL = 'http://localhost:8080';
const CACHE_DURATION = 5 * 60 * 1000; // 5 minutes
const BATCH_DELAY = 100; // 100ms delay for batching
const DEFAULT_PAGE_SIZE = 50; // Default page size

interface CacheEntry<T> {
    data: T;
    timestamp: number;
    totalCount: number;
}

interface PaginatedResponse<T> {
    data: T[];
    totalCount: number;
    hasMore: boolean;
}

class ApiCache {
    private cache: Map<string, CacheEntry<any>> = new Map();
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
function debounce<T>(fn: (...args: any[]) => Promise<T>, delay: number) {
    let timeoutId: number | null = null;
    return async (...args: any[]): Promise<T> => {
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

export async function fetchIdentifications(
    page: number,
    pageSize: number,
    filters?: FilterParams,
): Promise<{ data: NFeIdentification[]; totalCount: number; totalPages: number }> {
    try {
        console.log(
            `[API] Fetching identifications - page: ${page}, limit: ${pageSize}, filters:`,
            filters,
        );

        // Build query parameters
        const params = new URLSearchParams({
            page: page.toString(),
            page_size: pageSize.toString(),
        });

        // Add filter parameters if provided
        if (filters) {
            if (filters.natOp) params.append('nat_op', filters.natOp);
            if (filters.nNF) params.append('n_nf', filters.nNF);
            if (filters.tpNF) params.append('tp_nf', filters.tpNF);
            if (filters.dhEmi) params.append('dh_emi', filters.dhEmi);
            if (filters.searchQuery) params.append('search', filters.searchQuery);
        }

        const response = await fetch(
            `${API_BASE_URL}/api/nfe-identifications?${params.toString()}`,
        );

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        console.log(
            `[API] Received ${data.data.length} items out of ${data.totalCount} total`,
        );
        return data;
    } catch (error) {
        console.error('[API] Error fetching identifications:', error);
        throw error;
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

export async function updateIdentification(
    id: string,
    data: Partial<NFeIdentification>,
): Promise<NFeIdentification> {
    try {
        console.log("[API] Updating identification:", { id, data });
        const response = await fetch(`${API_BASE_URL}/identifications/${id}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
                Accept: "application/json",
            },
            body: JSON.stringify(data),
        });

        if (!response.ok) {
            const errorText = await response.text();
            console.error("[API] Update failed:", {
                status: response.status,
                statusText: response.statusText,
                error: errorText,
            });
            throw new Error(
                `Failed to update identification: ${response.status} ${response.statusText} - ${errorText}`,
            );
        }

        const updatedItem = await response.json();
        console.log("[API] Update successful:", updatedItem);
        return updatedItem;
    } catch (error) {
        console.error("[API] Update error:", error);
        metricsService.recordOperation("update_identification_error", {
            duration: Date.now() - Date.now(),
            details: { error: error instanceof Error ? error.message : "Unknown error" },
        });
        throw error;
    }
}

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