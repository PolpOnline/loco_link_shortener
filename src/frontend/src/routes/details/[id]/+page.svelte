<script lang="ts">
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<main>
	<div class="w-90 mx-auto">
		<h1>{data.name}</h1>

		<p>{data.original}</p>

		<p>{data.created_at}</p>

		<p>Displaying {data.clicks.length} clicks</p>

		{#each data.clicks as click}
			<p>{click.clicked_at} - {click.address} - {click.user_agent}</p>
		{/each}
	</div>
</main>
