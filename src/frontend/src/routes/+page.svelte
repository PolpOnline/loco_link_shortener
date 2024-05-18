<svelte:head>
	<title>Link Shortener</title>
</svelte:head>

<script lang="ts">
	import { InputShorten, LinksView } from '$components/index';
	import type { ListResponse } from '$lib/models';
	import type { PageData } from './$types';

	export let data: PageData;

	// @ts-ignore
	const listResponse: ListResponse = data.listResponse;
</script>

<main>
	<div class="w-90 mx-auto">
		<div class="my-3">
			<InputShorten />
		</div>
		<div class="my-3">
			<LinksView links={listResponse.links} />
		</div>
	</div>
</main>
