<script lang="ts">
	import { jwt } from '$lib/stores/auth';
	import { page } from '$app/stores';

	const token = $page.data.t;

	// Gets the jwt token from the oauth2/protected code
	async function getJWT() {
		if (!token) {
			Error('Bad Request: No token found');
		}

		jwt.set(token);

		window.location.href = '/';
	}

	getJWT();
</script>

<div class="d-flex flex-column justify-content-center align-items-center vh-100">
	<h1>Logging you in...</h1>
</div>
