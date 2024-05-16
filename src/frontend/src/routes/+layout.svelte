<script lang="ts">
	import '../scss/app.scss';
	import { ListErrors, Navbar } from '$components/index';

	import { onMount } from 'svelte';

	onMount(async () => {
		await import('bootstrap');
	});
</script>


<div class="text-body">
	<Navbar />

	<ListErrors />

	<slot />
</div>