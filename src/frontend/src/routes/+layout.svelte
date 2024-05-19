<svelte:head>
	<link rel="preconnect" href={base} />
</svelte:head>

<script lang="ts">
	import '../scss/app.scss';
	import 'unfonts.css';
	import { Footer, Loader, Navbar } from '$components/index';
	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';
	import { cubicIn, cubicOut } from 'svelte/easing';
	import { afterNavigate, beforeNavigate } from '$app/navigation';
	import { loginCheck } from '$lib/utils';
	import { base } from '$lib/api';

	onMount(async () => {
		await import('bootstrap');
	});

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


<div class="text-body bg-black actual-body">
	<Navbar />

	{#if isLoading}
		<Loader />
	{/if}

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
</style>