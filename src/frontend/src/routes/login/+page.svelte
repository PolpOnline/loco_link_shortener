<script lang="ts">
	import FlatColorIconsGoogle from '~icons/flat-color-icons/google';
	import { get } from '$lib/api';

	async function redirect() {
		const res = await get('oauth2/google');

		const text = await res.text();

		if (!res.ok) {
			throw Error(text);
		}

		window.location.href = text;
	}
</script>

<div class="d-flex flex-column justify-content-center align-items-center vh-100">
	<button class="btn btn-primary d-flex align-items-center" on:click={redirect}>
		<FlatColorIconsGoogle />
		<span class="ms-2">Login with Google</span>
	</button>
</div>
