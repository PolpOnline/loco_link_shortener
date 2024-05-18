<script lang="ts">
	import { LinkCard } from '$components';
	import type { Link } from '$lib/models';
	import { fly, type FlyParams } from 'svelte/transition';
	import { cubicIn } from 'svelte/easing';

	const flyOptions: FlyParams = {
		duration: 500,
		easing: cubicIn,
		x: '-25%'
	};

	export let links: Link[] = [];
</script>

<div>
	{#if links.length !== 0}
		<h1> Your links </h1>
	{/if}

	<div class="row">
		{#each links as link (link.shortened)}
			<div class="col-md-3 col-12 g-4 d-flex align-items-stretch justify-content-center" transition:fly={flyOptions}>
				<LinkCard image={link.image}
									original={link.original}
									shortened={link.shortened}
									created_at={new Date(link.created_at)}
									name={link.name}
				/>
			</div>
		{/each}
	</div>
</div>


