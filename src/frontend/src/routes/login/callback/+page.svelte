<script lang="ts">
	import { get } from '$lib/api';
	import { jwt } from '$lib/stores/auth';

	// Gets the jwt token from the oauth2/protected code
	async function getJWT() {
		const res = await get('oauth2/protected');

		const text = await res.text();

		if (!res.ok) {
			throw Error(text);
		}

		console.log(text);

		jwt.set(text);

		// window.location.href = '/';
	}

	getJWT();
</script>