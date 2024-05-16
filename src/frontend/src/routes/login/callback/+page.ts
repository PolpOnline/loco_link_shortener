export const prerender = false;
export const ssr = false;

export async function load({ params, url }) {
	let t = url.searchParams.get('t');
	return { t };
}
