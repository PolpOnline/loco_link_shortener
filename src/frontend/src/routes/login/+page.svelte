<script lang="ts">
	import FlatColorIconsGoogle from '~icons/flat-color-icons/google';
	import { base, send } from '$lib/api';

	async function goToLogin() {
		let res = await send({ method: 'GET', path: 'oauth2/google', credentialsRequired: true });

		window.location.href = await res.text();
	}
</script>

<svelte:head>
	<title>Login</title>
	<link rel="preconnect" href={base} crossorigin="anonymous" />
</svelte:head>

<div class="d-flex flex-column justify-content-center align-items-center vh-100">
	<button class="btn btn-outline-primary d-flex align-items-center" onclick={goToLogin}>
		<FlatColorIconsGoogle />
		<span class="ms-2">Login with Google</span>
	</button>
</div>

<style lang="scss">
	.btn {
		&:hover,
		&:focus,
		&:active {
			background-color: transparent;
		}
	}
</style>
