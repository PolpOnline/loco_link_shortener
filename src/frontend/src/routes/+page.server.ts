import type { ListResponse } from '$lib/models';
import { API_URL } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const listResponse = await fetch(`${API_URL}/list`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	}).then((res) => res.json() as Promise<ListResponse>);

	return {
		listResponse
	};
};
