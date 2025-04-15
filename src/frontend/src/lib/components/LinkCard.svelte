<script lang="ts">
	import { LinkBrief } from '$components';
	import HeroiconsLink from '~icons/heroicons/link';

	let detailsPage = `details/${shortened}`;
	let imageAlt = `Image for ${name}`;

	import { Image } from '@unpic/svelte';
	interface Props {
		name: string;
		image: string | undefined;
		original: string;
		shortened: string;
		created_at: Date;
	}

	let { name, image, original, shortened, created_at }: Props = $props();
</script>

<div class="card glass w-100">
	{#if image}
		<Image alt={imageAlt} class="card-img-top" src={image} />
	{:else}
		<div
			class="
						card-img-top
						aspect-ratio-16-9
						position-relative
						w-100
						d-flex
						align-items-center
						justify-content-center
            bg-body
            text-white
						rounded"
		>
			<HeroiconsLink class="fs-1" />
		</div>
	{/if}
	<div class="card-body d-flex flex-column align-items-baseline">
		<h5 class="card-title">{name}</h5>
		<div class="card-text mb-3">
			<LinkBrief createdAt={created_at} {original} />
		</div>
		<a class="btn btn-primary mt-auto w-100" data-sveltekit-preload-data href={detailsPage}>Info</a>
	</div>
</div>
