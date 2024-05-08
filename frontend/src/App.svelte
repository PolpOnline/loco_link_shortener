<script lang="ts">
	import Navbar from './lib/Navbar.svelte';
	import InputShorten from './lib/InputShorten.svelte';
	import LinksView from './lib/LinksView.svelte';
</script>

<Navbar />

<main>
	<div class="my-3 w-90 mx-auto">
		<InputShorten />
		<LinksView />
	</div>
</main>
