<script lang="ts">
	import '../scss/app.scss';
	import { Navbar, ListErrors } from '$components/index';

	import { onMount } from 'svelte';

	onMount(async () => {
		await import('bootstrap');
	});
</script>


<div class="bg-black text-body vh-100 vw-100" data-bs-theme="dark">
	<Navbar />

	<ListErrors />

	<slot />
</div>