import { API_URL } from '$lib/api';

export async function DELETE({ request, fetch }) {
	const res = await fetch(`${API_URL}/delete`, {
		method: 'DELETE',
		headers: {
			'Content-Type': 'application/json'
		},
		body: await request.text()
	});

	return new Response(await res.text(), { status: res.status });
}
