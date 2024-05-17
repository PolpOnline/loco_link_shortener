<script lang="ts">
	import { slide } from 'svelte/transition';
	import { cubicIn, cubicOut } from 'svelte/easing';


	let transitionIn = { easing: cubicOut, duration: 300 };
	let transitionOut = { easing: cubicIn, duration: 300 };
</script>

<div class="d-flex justify-content-center" in:slide={transitionIn} out:slide={transitionOut}>
	<div class="spinner-grow" role="status">
		<span class="visually-hidden">Loading...</span>
	</div>
</div>