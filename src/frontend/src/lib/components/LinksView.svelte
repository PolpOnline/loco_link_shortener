<script lang="ts">
	import { LinkCard } from '$components';

	// let cards = [];

	let mockData = [
		{
			name: 'Google',
			image: 'https://picsum.photos/1920/1080',
			original: 'https://www.google.com',
			shortened: 'abc123',
			createdAt: new Date()
		},
		{
			name: 'Bing',
			image: undefined,
			original: 'https://www.bing.com',
			shortened: 'def456',
			createdAt: new Date()
		}
	];

</script>

<div>
	<h1 class="text-body"> Your links </h1>

	<div class="row">
		{#each mockData as card}
			<div class="col-md-4 col-12 d-flex align-items-stretch g-4">
				<LinkCard image={card.image}
									original={card.original}
									shortened={card.shortened}
									createdAt={card.createdAt}
									name={card.name}
				/>
			</div>
		{/each}
	</div>
</div>


