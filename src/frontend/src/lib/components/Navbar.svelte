<script lang="ts">
	import HeroiconsLink from '~icons/heroicons/link';
</script>

<nav class="navbar navbar-expand-lg bg-transparent">
	<div class="container-fluid">
		<a class="navbar-brand" href="/">
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
				<li class="nav-item">
					<a aria-current="page" class="nav-link active" href="/">Home</a>
				</li>
			</ul>
		</div>
	</div>
</nav>