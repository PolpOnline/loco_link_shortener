import { error } from '@sveltejs/kit';
import { dev } from '$app/environment';

export let base = 'http://localhost:3000';

if (!dev) {
	base = 'https://s.polp.online';
}

let apiBase = base + '/api';

type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE';

type Headers = {
	'Content-Type'?: 'application/json' | 'text/plain' | 'multipart/form-data';
	Authorization?: string;
};

interface RequestOptions extends RequestInit {
	method: HttpMethod;
	headers: Headers;
	body?: string;
}

export type customFetchType = (
	input: string | URL | globalThis.Request,
	init?: RequestInit
) => Promise<Response>;

interface SendOptions {
	method: HttpMethod;
	path: string;
	data?: any; // Consider replacing 'any' with a more specific type
	token?: string;
	// Whether to include cookies in the request
	credentialsRequired?: boolean;
	// A function to use a custom fetch implementation
	customFetch?: customFetchType;
}

export async function send({
	method,
	path,
	data,
	token,
	credentialsRequired = false,
	customFetch = fetch
}: SendOptions): Promise<any> {
	const opts: RequestOptions = { method, headers: {} };

	if (data) {
		opts.headers['Content-Type'] = 'application/json';
		opts.body = JSON.stringify(data);
	}

	if (token) {
		opts.headers['Authorization'] = `Bearer ${token}`;
	}

	if (credentialsRequired) {
		opts.credentials = 'include';
	}

	const res = await customFetch(`${apiBase}/${path}`, opts);
	if (res.ok || res.status === 422) {
		// if the response is not a json, return the response object
		const text = await res.text();

		try {
			return JSON.parse(text);
		} catch (e) {
			return text;
		}
	}

	throw error(res.status);
}
