<script lang="ts">
	import HeroiconsLink from '~icons/heroicons/link';
	import { NavbarItem } from '$components/index';
	import LineMdGithub from '~icons/line-md/github';
</script>

<nav class="navbar navbar-expand-lg bg-transparent">
	<div class="container-fluid">
		<a class="navbar-brand p-0" href="/">
			<HeroiconsLink class="me-2" />
			Link Shortener
		</a>
		<button aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation"
						class="navbar-toggler"
						data-bs-target="#navbarSupportedContent" data-bs-toggle="collapse" type="button">
			<span class="navbar-toggler-icon"></span>
		</button>
		<div class="collapse navbar-collapse" id="navbarSupportedContent">
			<ul class="navbar-nav me-auto mb-2 mb-lg-0">
				<NavbarItem href="/" text="Home" />
				<NavbarItem href="/about" text="About" />
			</ul>
		</div>
		<div class="d-flex justify-content-end">
			<a class="text-white" href="https://github.com/PolpOnline/loco_link_shortener" target="_blank">
				<LineMdGithub class="fs-3" />
			</a>
		</div>
	</div>
</nav>