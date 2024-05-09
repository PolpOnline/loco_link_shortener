<script lang="ts">
	import { DefaultLinkImage, LinkBrief } from '$components';

	export let name: string;
	export let image: string | undefined;
	export let original: string;
	export let shortened: string;
	export let createdAt: Date;

	let detailsPage = `details/${shortened}`;
	let imageAlt = `Image for ${name}`;
</script>

<div class="card">
	{#if image}
		<img alt={imageAlt} class="card-img-top" src={image} />
	{:else}
		<div class="w-100">
			<DefaultLinkImage />
		</div>
	{/if}
	<div class="card-body d-flex flex-column align-items-baseline">
		<h5 class="card-title">{name}</h5>
		<p class="card-text">
			<LinkBrief createdAt={createdAt} original={original} />
		</p>
		<a class="btn btn-primary mt-auto w-100" href={detailsPage}>Info</a>
	</div>
</div>