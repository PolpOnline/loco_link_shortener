<script lang="ts">
	import { InputShorten, LinksView } from '$components/index';
	import type { PageData } from './$types';

	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();
</script>

<svelte:head>
	<title>Link Shortener</title>
</svelte:head>

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
