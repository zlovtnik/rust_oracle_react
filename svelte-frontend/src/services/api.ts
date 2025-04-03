import type { NFeIdentification } from '../types/nfeTypes';

const API_BASE_URL = 'http://localhost:8080/api';

export async function fetchIdentifications(): Promise<NFeIdentification[]> {
    const response = await fetch(`${API_BASE_URL}/identifications`);
    if (!response.ok) {
        throw new Error('Failed to fetch identifications');
    }
    return response.json();
}

export async function createIdentification(data: Omit<NFeIdentification, 'internal_key'>): Promise<NFeIdentification> {
    console.log('Sending data to API:', data);
    console.log('JSON stringified:', JSON.stringify(data, null, 2));

    const requestBody = JSON.stringify(data);
    console.log('Request body:', requestBody);

    const response = await fetch(`${API_BASE_URL}/identifications`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json',
        },
        body: requestBody,
    });

    console.log('Response status:', response.status);
    console.log('Response headers:', Object.fromEntries(response.headers.entries()));

    if (!response.ok) {
        const errorText = await response.text();
        console.error('Error response:', errorText);
        throw new Error(`Failed to create identification: ${errorText}`);
    }

    const responseData = await response.json();
    console.log('Response data:', responseData);
    return responseData;
}

export async function updateIdentification(id: string, data: NFeIdentification): Promise<NFeIdentification> {
    const response = await fetch(`${API_BASE_URL}/identifications/${id}`, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    });
    if (!response.ok) {
        throw new Error('Failed to update identification');
    }
    return response.json();
}

export async function deleteIdentification(id: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/identifications/${id}`, {
        method: 'DELETE',
    });
    if (!response.ok) {
        throw new Error('Failed to delete identification');
    }
} 