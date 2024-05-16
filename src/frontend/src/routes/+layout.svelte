<script lang="ts">
	import '../scss/app.scss';
	import { ListErrors, Navbar } from '$components/index';

	import { onMount } from 'svelte';
	import { get as getStore } from 'svelte/store';
	import { jwt } from '$lib/stores/auth';

	onMount(async () => {
		await import('bootstrap');
	});

	async function loginCheck() {
		if (!getStore(jwt)) {
			window.location.href = '/login';
		}
	}
</script>


<div class="text-body">
	<Navbar />

	<ListErrors />

	<slot />
</div>