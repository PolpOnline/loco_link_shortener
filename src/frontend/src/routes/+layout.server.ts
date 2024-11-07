import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ url, locals }) => {
	const { pathname } = url;

	return {
		loginStatus: locals.loginStatus,
		pathname
	};
};
