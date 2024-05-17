import type { PageLoad } from '../../.svelte-kit/types/src/routes/$types';

export const load: PageLoad = async ({ url }: { url: URL }) => {
	const { pathname } = url;

	return {
		pathname
	};
};
