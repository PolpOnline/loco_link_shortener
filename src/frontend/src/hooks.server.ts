import type { HandleFetch, ResolveOptions, Handle } from '@sveltejs/kit';
import { API_URL } from '$lib/api';
import cookie from 'cookie';
import type { LoginStatus } from './app';

// Forwards all cookies to the API, see https://kit.svelte.dev/docs/hooks#server-hooks-handlefetch
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	const isApiRequest = request.url.startsWith(API_URL);

	if (!isApiRequest) {
		return fetch(request);
	}

	// Check if client has the JWT cookie, and if so, forward it to the API
	const cookies = event.request.headers.get('cookie');
	if (!cookies) {
		return fetch(request);
	}

	const jwtCookie = cookie.parse(cookies).jwt;

	if (jwtCookie) {
		request.headers.set('Authorization', `Bearer ${jwtCookie}`);
	}

	return fetch(request);
};

// noinspection JSUnusedGlobalSymbols
export const resolveOptions: ResolveOptions = {
	preload: ({ type }) => type === 'font' || type === 'js' || type === 'css' || type === 'asset'
};

export const handle: Handle = async ({ event, resolve }) => {
	const requestedPath = event.url.pathname;

	event.locals.loginStatus = event.cookies.get('jwt') ? 'logged_in' : ('logged_out' as LoginStatus);

	if (requestedPath.startsWith('/login')) {
		return await resolve(event, resolveOptions);
	}

	if (event.locals.loginStatus === 'logged_out') {
		return new Response(null, {
			status: 302,
			headers: { location: '/login' }
		});
	}

	return await resolve(event, resolveOptions);
};
