<script lang="ts">
	import type { PageServerData } from './$types';

	// export interface InfoLinkView {   name: string   original: string   clicks: InfoClick[]   created_at: string
	//export interface InfoClick {   clicked_at: string   address: string   user_agent?: string

	export let data: PageServerData;

	console.log(data);
</script>

<main>
	<div class="w-90">
		<h1>{data.name}</h1>

		<p>{data.original}</p>

		<p>{data.created_at}</p>

		<p>{data.clicks.length} clicks</p>

		{#each data.clicks as click}
			<p>{click.clicked_at} - {click.address} - {click.user_agent}</p>
		{/each}

	</div>
</main>
