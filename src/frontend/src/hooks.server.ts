import type { Handle, HandleFetch, ResolveOptions } from '@sveltejs/kit';
import { API_URL } from '$lib/api';
import cookie from 'cookie';
import type { LoginStatus } from './app';
import { default as setCookieParser } from 'set-cookie-parser';

// Cookie max age in seconds (400 days)
export const COOKIE_ABSOLUTE_MAX_AGE = 34560000;

// Forwards all cookies to the API, see https://kit.svelte.dev/docs/hooks#server-hooks-handlefetch
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	const isApiRequest = request.url.startsWith(API_URL);

	if (!isApiRequest) {
		return fetch(request);
	}

	// Check if the client has the JWT cookie, and if so, forward it to the API
	const cookies = event.request.headers.get('cookie') || undefined;
	const parsedCookies = cookies ? cookie.parse(cookies) : undefined;

	// If the client has a JWT cookie, forward it to the API in the Authorization header
	if (parsedCookies) {
		const jwtCookie = parsedCookies.jwt;

		if (jwtCookie) {
			request.headers.set('Authorization', `Bearer ${jwtCookie}`);
		}

		// Forward all other cookies to the API
		const mergedCookies = Object.entries(parsedCookies)
			// Filter the jwt cookie as we've already handled it
			.filter(([name]) => name !== 'jwt')
			// Filter out cookies with undefined values
			.filter((pair): pair is [string, string] => pair[1] !== undefined)
			.map(([name, value]) => cookie.serialize(name, value))
			.join('; ');

		request.headers.set('Cookie', mergedCookies);
	}

	const res = await fetch(request);

	// If the API wants to set any cookies, forward them to the client
	const setCookieHeader = res.headers.getSetCookie();

	if (!setCookieHeader) {
		return res;
	}

	const setCookies = setCookieParser.parse(setCookieHeader);

	// Response did not contain a set-cookie header
	if (!setCookies) {
		return res;
	}

	// Forward the set-cookie header to the client
	setCookies.forEach((cookie) => {
		event.cookies.set(cookie.name, cookie.value, {
			// @ts-ignore
			sameSite: cookie.sameSite || 'strict',
			path: cookie.path || '/',
			maxAge: cookie.maxAge || COOKIE_ABSOLUTE_MAX_AGE,
			httpOnly: true,
			secure: true
		});
	});

	return res;
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
