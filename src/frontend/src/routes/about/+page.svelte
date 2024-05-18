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
	<p class="my-4">
		The frontend is built with SvelteKit and Bootstrap 5.
	</p>

	<div class="container">
		<div class="row">
			<div class="col-12 col-md-6 img-fluid d-flex align-items-center justify-content-center py-2">
				<Svelte --shadow-color="#FF3E00" class="uniform-height drop-shadow" />
			</div>
			<div class="col-12 col-md-6 img-fluid d-flex align-items-center justify-content-center py-2">
				<enhanced:img alt="Bootstrap logo" class="uniform-height drop-shadow" src={Bootstrap}
											style="--shadow-color: #7610F6" />
			</div>
		</div>
	</div>

	<p class="my-4">
		The backend is built with the Loco Framework, a Rust web framework that uses Axum and SeaORM.
	</p>

	<div class="container">
		<div class="row">
			<div class="col-12 col-md-4 d-flex align-items-center justify-content-center py-2">
				<enhanced:img alt="Loco logo" class="uniform-height drop-shadow" src={Loco} style="--shadow-color: #FC9E12" />
			</div>
			<div class="col-12 col-md-4 img-fluid d-flex align-items-center justify-content-center py-2">
				<Axum --shadow-color="white" class="uniform-height drop-shadow" />
			</div>
			<div class="col-12 col-md-4 img-fluid d-flex align-items-center justify-content-center">
				<enhanced:img alt="SeaORM logo" class="uniform-height drop-shadow" src={SeaORM}
											style="--shadow-color: #2564D8" />
			</div>
		</div>
	</div>

	<p class="my-4">
		The database is PostgreSQL.
	</p>

	<div class="container">
		<div class="row">
			<div class="col-12 img-fluid d-flex align-items-center justify-content-center">
				<PostgreSQL --shadow-color="white" class="uniform-height drop-shadow" />
			</div>
		</div>
	</div>

	<p class="my-4">
		The application is deployed on Railway.
	</p>

	<div class="container mb-5">
		<div class="row">
			<div class="col-12 img-fluid d-flex align-items-center justify-content-center">
				<Railway --shadow-color="white" class="uniform-height drop-shadow" />
			</div>
		</div>
	</div>
</div>

<style lang="scss">
  :global(.uniform-height) {
    max-height: 200px;
    max-width: 200px;
    object-fit: contain;
  }

  :global(.drop-shadow) {
    transition: filter 1s;
  }

  :global(.drop-shadow:hover) {
    filter: drop-shadow(0 0 1rem var(--shadow-color));
  }
</style>