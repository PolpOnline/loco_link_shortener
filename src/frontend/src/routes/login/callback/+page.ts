export const prerender = false;
export const ssr = false;

export async function load({ url }) {
	const t = url.searchParams.get('t');
	return { t };
}
