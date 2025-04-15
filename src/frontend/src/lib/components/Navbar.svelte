<script lang="ts">
	import HeroiconsLink from '~icons/heroicons/link';
	import { NavbarItem } from '$components/index';
	import LineMdGithub from '~icons/line-md/github';
	import { jwt } from '$lib/stores/auth';

	let isUserLoggedIn = $derived($jwt !== '');

	let href = $derived(isUserLoggedIn ? '/' : '');
</script>

<nav class="navbar navbar-expand-lg bg-transparent">
	<div class="container-fluid">
		<a class="navbar-brand p-0" {href}>
			<HeroiconsLink --shadow-color="white" class="me-2 my-auto drop-shadow" />
			<span class="colored-text">Link Shortener</span>
		</a>
		{#if isUserLoggedIn}
			<button
				aria-controls="navbarSupportedContent"
				aria-expanded="false"
				aria-label="Toggle navigation"
				class="navbar-toggler"
				data-bs-target="#navbarSupportedContent"
				data-bs-toggle="collapse"
				type="button"
			>
				<span class="navbar-toggler-icon"></span>
			</button>
			<div class="collapse navbar-collapse" id="navbarSupportedContent">
				<ul class="navbar-nav me-auto mb-2 mb-lg-0">
					<NavbarItem href="/" text="Home" />
					<NavbarItem href="/about" text="About" />
				</ul>
				<a
					class="text-white"
					href="https://github.com/PolpOnline/loco_link_shortener"
					target="_blank"
				>
					<LineMdGithub --shadow-color="white" class="fs-3 drop-shadow" />
				</a>
			</div>
		{/if}
	</div>
</nav>

<style>
	.colored-text {
		font-weight: 600;
		background: -webkit-linear-gradient(0deg, #0055ff 0%, #08f794 50%, #0055ff 100%);
		background-size: 200% auto;
		background-clip: text;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		color: transparent;
		animation: glint 5s linear infinite;
	}

	@keyframes glint {
		to {
			background-position: 200% center;
		}
	}
</style>
