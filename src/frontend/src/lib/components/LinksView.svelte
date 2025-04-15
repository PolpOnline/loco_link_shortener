<script lang="ts">
	import { LinkCard } from '$components';
	import type { Link } from '$lib/models';
	import { fly, type FlyParams } from 'svelte/transition';
	import { cubicIn } from 'svelte/easing';
	import { mediaQueryStore } from '$lib/stores/screenWidth';
	import { flip, type FlipParams } from 'svelte/animate';

	const smallThreshold = 768; // Bootstrap's md breakpoint

	const small = mediaQueryStore(`(max-width: ${smallThreshold}px)`);

	let flyOptions: FlyParams = $derived(
		$small
			? {
					duration: 500,
					easing: cubicIn,
					y: '-100%'
				}
			: {
					duration: 500,
					easing: cubicIn,
					x: '-100%'
				}
	);

	const flipOptions: FlipParams = {
		duration: 500,
		easing: cubicIn
	};

	interface Props {
		links?: Link[];
	}

	let { links = [] }: Props = $props();
</script>

<div>
	{#if links.length !== 0}
		<h1 class="mt-5 text-center custom-underline">Your links</h1>
	{/if}

	<div class="row">
		{#each links as link (link.shortened)}
			<div
				class="col-12 col-sm-6 col-md-4 col-lg-3 g-4 d-flex align-items-stretch justify-content-center"
				transition:fly={flyOptions}
				animate:flip={flipOptions}
			>
				<LinkCard
					image={link.image}
					original={link.original}
					shortened={link.shortened}
					created_at={new Date(link.created_at + 'Z')}
					name={link.name}
				/>
			</div>
		{/each}
	</div>
</div>

<style lang="scss">
	.custom-underline {
		user-select: none;
		--offset: 1em;
		text-decoration: underline;
		text-underline-offset: var(--offset);
		margin-bottom: var(--offset);
		text-decoration-thickness: 1px;
		text-decoration-color: rgba(var(--bs-body-color-rgb), 0.25);
	}
</style>
