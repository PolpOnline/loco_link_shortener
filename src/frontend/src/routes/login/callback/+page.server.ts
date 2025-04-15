import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
	let token = url.searchParams.get('t');

	if (!token) {
		throw new Error('No token provided');
	}

	return { token };
};
