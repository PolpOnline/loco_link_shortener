import { API_URL } from '$lib/api';
import type { PageServerLoad } from '../$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const gotoUrl = await fetch(`${API_URL}/oauth2/google`, {
		method: 'GET'
	}).then((res) => res.text());

	return {
		gotoUrl
	};
};
