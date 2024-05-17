<script lang="ts">
	import HeroiconsLink from '~icons/heroicons/link';
	import HeroiconsClock from '~icons/heroicons/clock';

	export let createdAt: Date;
	export let original: string;

	let domainOriginal = new URL(original).hostname;

	let createdAtString = createdAt.toLocaleString();
</script>

<div class="w-100">
	<div class="d-flex align-items-center">
		<HeroiconsLink class="me-2" />
		<div>
			{domainOriginal}
		</div>
	</div>

	<div class="d-flex align-items-center">
		<HeroiconsClock class="me-2" />
		<div>
			{createdAtString}
		</div>
	</div>
</div>