import type { InfoLinkView } from '$lib/models';

let mockData: InfoLinkView = {
	name: 'hello-world',
	original: 'https://example.com',
	clicks: [
		{
			clicked_at: '2021-08-01T00:00:00Z',
			address: '192.168.1.1',
			user_agent:
				'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
		},
		{
			clicked_at: '2021-08-01T00:00:00Z',
			address: '192.168.1.1'
		}
	],
	created_at: '2021-08-01T00:00:00Z'
};

export const load = () => {
	return {
		mockData
	};
};
