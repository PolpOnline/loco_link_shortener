// noinspection DuplicatedCode

import type { InfoLinkView } from '$lib/models';
import { API_URL } from '$lib/api';
import type { PageServerLoad } from './$types';

const loadInfo = async (fetch: typeof window.fetch, shortened: string) => {
	return fetch(`${API_URL}/info/${shortened}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	}).then((res) => res.json() as Promise<InfoLinkView>);
};

export const load: PageServerLoad = async ({ fetch, params }) => {
	// @ts-ignore
	const shortened = params.id;

	const info = await loadInfo(fetch, shortened);

	return {
		info
	};
};
