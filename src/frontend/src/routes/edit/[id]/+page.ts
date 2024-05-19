// noinspection DuplicatedCode

import type { InfoLinkView } from '$lib/models';
import { type customFetchType, send } from '$lib/api';
import { get as storeGet } from 'svelte/store';
import { jwt } from '$lib/stores/auth';
import type { PageLoad } from './$types';

export const ssr = false;
export const prerender = false;

const loadInfo = async (fetch: customFetchType, shortened: string) => {
	const token = storeGet(jwt);

	const res: Response = await send({
		method: 'GET',
		path: `info/${shortened}`,
		token,
		customFetch: fetch
	});

	const listResponse: InfoLinkView = await res.json();

	return listResponse;
};

export const load: PageLoad = async ({ fetch, params }) => {
	// @ts-ignore
	const shortened = params.id;

	const info = await loadInfo(fetch, shortened);

	return {
		info
	};
};
