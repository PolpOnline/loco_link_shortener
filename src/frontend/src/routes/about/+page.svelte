<script lang="ts">
	import { UnderlinedText } from '$components';
	import Bootstrap from '$images/Bootstrap.png?enhanced';
	import Loco from '$images/Loco.png?enhanced';
	import SeaORM from '$images/SeaORM.png?enhanced';
	import PostgreSQL from '$images/PostgreSQL.svelte';
	import Railway from '$images/Railway.svelte';
	import Axum from '$images/Axum.svelte';
	import Svelte from '$images/Svelte.svelte';
	import CloudflarePages from '$images/CloudflarePages.svelte';

	const svelteColor = '#FF3E00';
	const bootstrapColor = '#7610F6';
	const locoColor = '#FC9E12';
	const axumColor = 'white';
	const seaORMColor = '#2564D8';
	const postgreSQLColor = '#336791';
	const railwayColor = 'white';
	const cloudflarePagesColor = '#FFA500';
</script>

<svelte:head>
	<title>About</title>
</svelte:head>

<h1 class="text-center">About</h1>

<hr class="w-75 mx-auto my-3" />

<div class="text-justified w-75 mx-auto">
	<p class="my-4 text-center">
		The frontend is built with
		<UnderlinedText color={svelteColor}>SvelteKit</UnderlinedText>
		and
		<span
			><UnderlinedText class="ms-1 me-0" color={bootstrapColor}>Bootstrap 5</UnderlinedText>.</span
		>
	</p>

	<div class="container w-50">
		<div class="row">
			<div class="col-12 col-md-6 img-fluid d-flex align-items-center justify-content-center py-2">
				<a
					class="flex-a-fix d-flex align-items-center justify-content-center"
					href="https://svelte.dev/"
					target="_blank"
				>
					<Svelte --shadow-color={svelteColor} class="uniform-height drop-shadow" />
				</a>
			</div>
			<div class="col-12 col-md-6 img-fluid d-flex align-items-center justify-content-center py-2">
				<a href="https://getbootstrap.com/" target="_blank">
					<enhanced:img
						alt="Bootstrap logo"
						class="uniform-height drop-shadow"
						src={Bootstrap}
						style={`--shadow-color: ${bootstrapColor}`}
					/>
				</a>
			</div>
		</div>
	</div>

	<p class="my-4 text-center">
		The backend is built with the
		<UnderlinedText color={locoColor}>Loco</UnderlinedText>
		Framework, a Rust web framework that uses
		<UnderlinedText color={axumColor}>Axum</UnderlinedText>
		and
		<span><UnderlinedText color={seaORMColor}>SeaORM</UnderlinedText>.</span>
	</p>

	<div class="container w-75">
		<div class="row">
			<div class="col-12 col-md-4 d-flex align-items-center justify-content-center py-2">
				<a href="https://loco.rs/" target="_blank">
					<enhanced:img
						alt="Loco logo"
						class="uniform-height drop-shadow"
						src={Loco}
						style={`--shadow-color: ${locoColor}`}
					/>
				</a>
			</div>
			<div class="col-12 col-md-4 img-fluid d-flex align-items-center justify-content-center py-2">
				<a href="https://github.com/tokio-rs/axum" target="_blank">
					<Axum --shadow-color={axumColor} class="uniform-height drop-shadow" />
				</a>
			</div>
			<div class="col-12 col-md-4 img-fluid d-flex align-items-center justify-content-center">
				<a href="https://www.sea-ql.org/SeaORM/" target="_blank">
					<enhanced:img
						alt="SeaORM logo"
						class="uniform-height drop-shadow"
						src={SeaORM}
						style={`--shadow-color: ${seaORMColor}`}
					/>
				</a>
			</div>
		</div>
	</div>

	<p class="my-4 text-center">
		The database is
		<span><UnderlinedText color={postgreSQLColor}>PostgreSQL</UnderlinedText>.</span>
	</p>

	<div class="container">
		<div class="row">
			<div class="col-12 img-fluid d-flex align-items-center justify-content-center">
				<a href="https://www.postgresql.org/" target="_blank">
					<PostgreSQL --shadow-color={postgreSQLColor} class="uniform-height drop-shadow" />
				</a>
			</div>
		</div>
	</div>

	<p class="my-4 text-center">
		The application is deployed on
		<UnderlinedText color={railwayColor}>Railway</UnderlinedText>
		and
		<span><UnderlinedText color={cloudflarePagesColor}>Cloudflare Pages</UnderlinedText>.</span>
	</p>

	<div class="container mb-5">
		<div class="row">
			<div class="col-12 col-md-6 img-fluid d-flex align-items-center justify-content-center py-2">
				<a href="https://railway.app/" target="_blank">
					<Railway --shadow-color={railwayColor} class="uniform-height drop-shadow" />
				</a>
			</div>
			<div class="col-12 col-md-6 img-fluid d-flex align-items-center justify-content-center py-2">
				<a href="https://pages.cloudflare.com/" target="_blank">
					<CloudflarePages
						--shadow-color={cloudflarePagesColor}
						class="uniform-height drop-shadow"
					/>
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

	.flex-a-fix {
		flex: 1 0 0;
	}
</style>
