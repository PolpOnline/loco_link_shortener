<svelte:head>
	<title>Logging you in...</title>
</svelte:head>

<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	const token = $page.data.token;

	onMount(async () => {
		if (token) {
			document.cookie = `jwt=${token}; path=/`;
		}

		await goto('/');
	})
</script>

<div class="d-flex flex-column justify-content-center align-items-center vh-100">
	<h1>Logging you in...</h1>
</div>
