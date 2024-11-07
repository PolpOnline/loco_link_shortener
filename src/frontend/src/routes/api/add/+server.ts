import { API_URL } from '$lib/api';

export async function POST({ request, fetch }) {
	const res = await fetch(`${API_URL}/add`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: await request.text()
	});

	return new Response(await res.text(), { status: res.status });
}
