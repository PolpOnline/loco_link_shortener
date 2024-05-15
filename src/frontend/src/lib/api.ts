import { error } from '@sveltejs/kit';
import { dev } from '$app/environment';

export let base = 'http://localhost:3000/api';

if (!dev) {
	base = 'https://s.polp.online/api';
}

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

interface SendOptions {
	method: HttpMethod;
	path: string;
	data?: any; // Consider replacing 'any' with a more specific type
	token?: string;
}

async function send({ method, path, data, token }: SendOptions): Promise<Response> {
	const opts: RequestOptions = { method, headers: {} };

	if (data) {
		opts.headers['Content-Type'] = 'application/json';
		opts.body = JSON.stringify(data);
	}

	if (token) {
		opts.headers['Authorization'] = `Bearer ${token}`;
	}

	opts.credentials = 'include';

	const res = await fetch(`${base}/${path}`, opts);
	if (res.ok || res.status === 422) {
		const text = await res.text();

		// if the response is not a json, return a custom Response object
		try {
			return JSON.parse(text);
		} catch (e) {
			return new Response(text);
		}
	}

	throw error(res.status);
}

export function get(path: string, token?: string) {
	return send({ method: 'GET', path, token });
}

export function del(path: string, token?: string) {
	return send({ method: 'DELETE', path, token });
}

export function post(path: string, data: any, token?: string) {
	return send({ method: 'POST', path, data, token });
}

export function put(path: string, data: any, token?: string) {
	return send({ method: 'PUT', path, data, token });
}
