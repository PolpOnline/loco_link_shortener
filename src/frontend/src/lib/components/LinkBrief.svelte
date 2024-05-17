<script lang="ts">
	import HeroiconsLink from '~icons/heroicons/link';
	import HeroiconsClock from '~icons/heroicons/clock';

	export let createdAt: Date;
	export let original: string;

	let createdAtString = createdAt.toLocaleString();
</script>

<div class="container-fluid">
	<div class="row">
		<div class="col-2">
			<HeroiconsLink class="me-2" />
		</div>
		<div class="col-10">
			{original}
		</div>
	</div>
	<div class="row">
		<div class="col-2">
			<HeroiconsClock class="me-2" />
		</div>
		<div class="col-10">
			{createdAtString}
		</div>
	</div>
</div>