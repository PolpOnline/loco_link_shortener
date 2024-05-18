<svelte:head>
	<title>About</title>
</svelte:head>

<script lang="ts">
	import Bootstrap from '$images/Bootstrap.png?enhanced';
	import Loco from '$images/Loco.png?enhanced';
	import SeaORM from '$images/SeaORM.png?enhanced';
	import PostgreSQL from '$images/PostgreSQL.svelte';
	import Railway from '$images/Railway.svelte';
	import Axum from '$images/Axum.svelte';
	import Svelte from '$images/Svelte.svelte';
</script>


<h1 class="text-center">About</h1>

<hr class="w-75 mx-auto my-3">

<div class="text-justified w-75 mx-auto">
	<p class="my-4 text-center">
		The frontend is built with SvelteKit and Bootstrap 5.
	</p>

	<div class="container w-50">
		<div class="row">
			<div class="col-12 col-md-6 img-fluid d-flex align-items-center justify-content-center py-2">
				<a class="flex-a-fix" href="https://svelte.dev/" target="_blank">
					<Svelte --shadow-color="#FF3E00" class="uniform-height drop-shadow" />
				</a>
			</div>
			<div class="col-12 col-md-6 img-fluid d-flex align-items-center justify-content-center py-2">
				<a href="https://getbootstrap.com/" target="_blank">
					<enhanced:img alt="Bootstrap logo" class="uniform-height drop-shadow" src={Bootstrap}
												style="--shadow-color: #7610F6" />
				</a>
			</div>
		</div>
	</div>

	<p class="my-4 text-center">
		The backend is built with the Loco Framework, a Rust web framework that uses Axum and SeaORM.
	</p>

	<div class="container w-75">
		<div class="row">
			<div class="col-12 col-md-4 d-flex align-items-center justify-content-center py-2">
				<a href="https://loco.rs/" target="_blank">
					<enhanced:img alt="Loco logo" class="uniform-height drop-shadow" src={Loco} style="--shadow-color: #FC9E12" />
				</a>
			</div>
			<div class="col-12 col-md-4 img-fluid d-flex align-items-center justify-content-center py-2">
				<a href="https://github.com/tokio-rs/axum" target="_blank">
					<Axum --shadow-color="white" class="uniform-height drop-shadow" />
				</a>
			</div>
			<div class="col-12 col-md-4 img-fluid d-flex align-items-center justify-content-center">
				<a href="https://www.sea-ql.org/SeaORM/" target="_blank">
					<enhanced:img alt="SeaORM logo" class="uniform-height drop-shadow" src={SeaORM}
												style="--shadow-color: #2564D8" />
				</a>
			</div>
		</div>
	</div>

	<p class="my-4 text-center">
		The database is PostgreSQL.
	</p>

	<div class="container">
		<div class="row">
			<div class="col-12 img-fluid d-flex align-items-center justify-content-center">
				<a href="https://www.postgresql.org/" target="_blank">
					<PostgreSQL --shadow-color="white" class="uniform-height drop-shadow" />
				</a>
			</div>
		</div>
	</div>

	<p class="my-4 text-center">
		The application is deployed on Railway.
	</p>

	<div class="container mb-5">
		<div class="row">
			<div class="col-12 img-fluid d-flex align-items-center justify-content-center">
				<a href="https://railway.app/" target="_blank">
					<Railway --shadow-color="white" class="uniform-height drop-shadow" />
				</a>
			</div>
		</div>
	</div>
</div>

<style lang="scss">
  :global(.uniform-height) {
    max-height: 150px;
    max-width: 150px;
    object-fit: contain;
  }

  :global(.drop-shadow) {
    transition: filter 1s;
  }

  :global(.drop-shadow:hover) {
    filter: drop-shadow(0 0 1rem var(--shadow-color));
  }

  .flex-a-fix {
    flex: 1 0 0;
  }
</style>