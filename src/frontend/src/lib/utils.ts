import { get as getStore } from 'svelte/store';
import { jwt } from '$lib/stores/auth';
import { goto } from '$app/navigation';

export async function loginCheck() {
	if (!isUserLoggedIn()) {
		await goto('/login');
	}
}

export function isUserLoggedIn() {
	return !!getStore(jwt);
}
