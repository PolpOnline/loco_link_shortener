import 'unplugin-icons/types/svelte';

export type LoginStatus = 'logged_out' | 'logged_in';

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			loginStatus: LoginStatus;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
