import type { InfoLinkView } from '$lib/models';
import { type customFetchType, send } from '$lib/api';
import { get as storeGet } from 'svelte/store';
import { jwt } from '$lib/stores/auth';
import type { PageLoad } from '../../../../.svelte-kit/types/src/routes/$types';

// let mockData: InfoLinkView = {
// 	name: 'hello-world',
// 	original: 'https://example.com',
// 	shortened: 'aFDSfe',
// 	clicks: [
// 		{
// 			clicked_at: new Date('2021-08-01T00:00:00Z'),
// 			address: '192.168.1.1',
// 			user_agent:
// 				'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
// 		},
// 		{
// 			clicked_at: new Date('2021-08-01T00:00:00Z'),
// 			address: '192.168.1.1'
// 		}
// 	],
// 	created_at: new Date('2021-08-01T00:00:00Z')
// };

export const ssr = false;
export const prerender = false;

const loadInfo = async (fetch: customFetchType, shortened: string) => {
	const token = storeGet(jwt);

	const listResponse: InfoLinkView = await send({
		method: 'GET',
		path: `info/${shortened}`,
		token,
		customFetch: fetch
	});

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
