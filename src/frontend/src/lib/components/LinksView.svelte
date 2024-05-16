<script lang="ts">
	import { LinkCard } from '$components';
	import type { Link } from '$lib/models';

	export let links: Link[] = [];
</script>

<div>
	<h1 class="text-body"> Your links </h1>

	<div class="row">
		{#each links as link}
			<div class="col-md-3 col-12 g-4 d-flex align-items-stretch justify-content-center">
				<!--	TODO: image -->
				<LinkCard image={undefined}
									original={link.original}
									shortened={link.shortened}
									created_at={link.created_at}
									name={link.name}
				/>
			</div>
		{/each}
	</div>
</div>


