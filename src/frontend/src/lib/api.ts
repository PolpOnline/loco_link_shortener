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

interface SendOptions {
	method: HttpMethod;
	path: string;
	data?: any; // Consider replacing 'any' with a more specific type
	token?: string;
	// Whether to include cookies in the request
	credentialsRequired?: boolean;
}

async function send({
	method,
	path,
	data,
	token,
	credentialsRequired = false
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

	const res = await fetch(`${apiBase}/${path}`, opts);
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

export function getWithCredentials(path: string, token?: string) {
	return send({ method: 'GET', path, token, credentialsRequired: true });
}

export function get(path: string, token?: string) {
	return send({ method: 'GET', path, token });
}

export function del(path: string, data: any, token?: string) {
	return send({ method: 'DELETE', path, data, token });
}

export function post(path: string, data: any, token?: string) {
	return send({ method: 'POST', path, data, token });
}

export function put(path: string, data: any, token?: string) {
	return send({ method: 'PUT', path, data, token });
}
