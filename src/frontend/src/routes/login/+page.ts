import { API_URL } from '$lib/api';
import type { PageLoad } from './$types';

export const ssr = false;

export const load: PageLoad = async ({ fetch }) => {
	const gotoUrl = await fetch(`${API_URL}/oauth2/google`, {
		method: 'GET'
	}).then((res) => res.text());

	return {
		gotoUrl
	};
};
