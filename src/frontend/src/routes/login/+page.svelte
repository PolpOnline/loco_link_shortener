<script lang="ts">
	import FlatColorIconsGoogle from '~icons/flat-color-icons/google';
	import { send } from '$lib/api';

	async function redirect() {
		window.location.href = await send({ method: 'GET', path: 'oauth2/google', credentialsRequired: true });
	}
</script>

<div class="d-flex flex-column justify-content-center align-items-center vh-100">
	<button class="btn btn-primary d-flex align-items-center" on:click={redirect}>
		<FlatColorIconsGoogle />
		<span class="ms-2">Login with Google</span>
	</button>
</div>
