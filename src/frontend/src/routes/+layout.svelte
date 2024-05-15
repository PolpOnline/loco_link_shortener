<script lang="ts">
	import '../scss/app.scss';
	import { Navbar } from '$components/index';
	import ListErrors from '$components/ListErrors.svelte';

	import { onMount } from 'svelte';

	onMount(async () => {
		await import('bootstrap');
	});
</script>


<div class="bg-black" data-bs-theme="dark">
	<Navbar />

	<ListErrors />

	<slot />
</div>