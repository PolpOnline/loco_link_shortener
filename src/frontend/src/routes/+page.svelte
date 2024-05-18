<svelte:head>
	<title>Link Shortener</title>
</svelte:head>

<script lang="ts">
	import { InputShorten, LinksView } from '$components/index';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<main>
	<div class="w-90 mx-auto">
		<div class="my-3">
			<InputShorten />
		</div>
		<div class="my-3">
			<LinksView links={data.listResponse.links} />
		</div>
	</div>
</main>
