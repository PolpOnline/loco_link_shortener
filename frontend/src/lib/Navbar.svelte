<script lang="ts">
	let isOpen = false;

	// function handleUpdate(e: CustomEvent<boolean>) {
	// 	isOpen = e.detail;
	// }
</script>

<nav class="navbar navbar-expand-md navbar-dark bg-dark">
	<div class="container-md">
		<a class="navbar-brand" href="/">Loco Link Shortener</a>
		<button class="navbar-toggler" on:click={() => (isOpen = !isOpen)} type="button">
			<span class="navbar-toggler-icon"></span>
		</button>
		<div class="collapse navbar-collapse" class:show={isOpen}>
			<ul class="navbar-nav ms-auto">
				<li class="nav-item">
					<a class="nav-link" href="/">Home</a>
				</li>
			</ul>
		</div>
	</div>
</nav>