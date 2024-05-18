<script lang="ts">
	import '../scss/app.scss';
	import { Footer, ListErrors, Loader, Navbar } from '$components/index';
	import { onMount } from 'svelte';
	import { get as getStore } from 'svelte/store';
	import { jwt } from '$lib/stores/auth';
	import { fly } from 'svelte/transition';
	import { cubicIn, cubicOut } from 'svelte/easing';
	import { afterNavigate, beforeNavigate, goto } from '$app/navigation';

	onMount(async () => {
		await import('bootstrap');
	});

	async function loginCheck() {
		if (!getStore(jwt)) {
			await goto('/login');
		}
	}

	loginCheck();

	export let data;

	const duration = 300;
	const delay = duration + 100;
	const y = 10;

	const transitionIn = { easing: cubicOut, y, duration, delay };
	const transitionOut = { easing: cubicIn, y: -y, duration };


	let isLoading = false;

	// Show loader only when navigating between internal pages
	beforeNavigate(({ to }) => (isLoading = !!to?.route.id));
	afterNavigate(() => (isLoading = false));
</script>


<div class="text-body poppins bg-black actual-body">
	<Navbar />

	{#if isLoading}
		<Loader />
	{/if}

	<ListErrors />

	{#key data.pathname}
		<div in:fly={transitionIn} out:fly={transitionOut}>
			<slot />
		</div>
	{/key}

	<Footer />
</div>

<style>
    .actual-body {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }

    .poppins {
        font-family: 'Poppins', sans-serif;
    }
</style>