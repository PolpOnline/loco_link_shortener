<script lang="ts">
	import { LinkCard } from '$components';

	// let cards = [];

	let mockData = [
		{
			name: 'Google',
			image: 'https://picsum.photos/1920/1080?1',
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
		},
		{
			name: 'DuckDuckGo',
			image: 'https://picsum.photos/1920/720?2',
			original: 'https://www.duckduckgo.com',
			shortened: 'ghi789',
			createdAt: new Date()
		},
		{
			name: 'Yahoo',
			image: 'https://picsum.photos/400/400?3',
			original: 'https://www.yahoo.com',
			shortened: 'jkl012',
			createdAt: new Date()
		}
	];

</script>

<div>
	<h1 class="text-body"> Your links </h1>

	<div class="row">
		{#each mockData as card}
			<div class="col-md-4 col-12 g-4 d-flex align-items-stretch justify-content-center">
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


