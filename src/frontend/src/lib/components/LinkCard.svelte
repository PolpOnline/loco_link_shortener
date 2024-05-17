<script lang="ts">
	import { LinkBrief } from '$components';
	import HeroiconsLink from '~icons/heroicons/link';

	export let name: string;
	export let image: string | undefined;
	export let original: string;
	export let shortened: string;
	export let created_at: Date;

	let detailsPage = `details/${shortened}`;
	let imageAlt = `Image for ${name}`;
</script>

<div class="card glass w-100">
	{#if image}
		<img alt={imageAlt} class="card-img-top" src={image} />
	{:else}
		<div class="
						card-img-top
						aspect-ratio-16-9
						position-relative
						w-100
						d-flex
						align-items-center
						justify-content-center
            bg-body
            text-white
						rounded-">
			<HeroiconsLink class="fs-1" />
		</div>
	{/if}
	<div class="card-body d-flex flex-column align-items-baseline">
		<h5 class="card-title">{name}</h5>
		<div class="card-text">
			<LinkBrief createdAt={created_at} original={original} />
		</div>
		<a class="btn btn-primary mt-5 w-100" data-sveltekit-preload-data href={detailsPage}>Info</a>
	</div>
</div>


<style lang="scss">
  .aspect-ratio-16-9 {
    aspect-ratio: 16 / 9; // This gives the div a 16:9 aspect ratio. Adjust as needed
  }
</style>