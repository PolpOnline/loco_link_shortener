<script lang="ts">
	import LinkBrief from './LinkBrief.svelte';

	export let name: string;
	export let image: string;
	export let original: string;
	export let shortened: string;
	export let createdAt: Date;

	let detailsPage = `details/${shortened}`;
	let imageAlt = `Image for ${name}`;
</script>

<div class="card">
	<img alt={imageAlt} class="card-img-top" src={image} />
	<div class="card-body d-flex flex-column align-items-baseline">
		<h5 class="card-title">{name}</h5>
		<p class="card-text">
			<LinkBrief createdAt={createdAt} original={original} />
		</p>
		<a class="btn btn-primary mt-auto w-100" href={detailsPage}>Info</a>
	</div>
</div>