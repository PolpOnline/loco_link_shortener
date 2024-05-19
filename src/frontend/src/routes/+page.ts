import type { ListResponse } from '$lib/models';
import { get as storeGet } from 'svelte/store';
import { jwt } from '$lib/stores/auth';
import { type customFetchType, send } from '$lib/api';
import type { PageLoad } from './$types';

export const ssr = false;
export const prerender = false;

const loadList = async ({ fetch }: { fetch: customFetchType }) => {
	const token = storeGet(jwt);

	const res: Response = await send({
		method: 'GET',
		path: 'list',
		token,
		customFetch: fetch
	});

	const listResponse: ListResponse = await res.json();

	return listResponse;
};

export const load: PageLoad = async ({ fetch }: { fetch: customFetchType }) => {
	const listResponse = await loadList({ fetch });

	return {
		listResponse
	};
};
