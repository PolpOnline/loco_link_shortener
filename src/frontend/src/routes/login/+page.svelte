<script lang="ts">
	import FlatColorIconsGoogle from '~icons/flat-color-icons/google';
	import { send } from '$lib/api';

	async function goToLogin() {
		window.location.href = await send({ method: 'GET', path: 'oauth2/google', credentialsRequired: true });
	}
</script>

<div class="d-flex flex-column justify-content-center align-items-center vh-100">
	<button class="btn btn-outline-primary d-flex align-items-center" on:click={goToLogin}>
		<FlatColorIconsGoogle />
		<span class="ms-2">Login with Google</span>
	</button>
</div>


<style lang="scss">
  @media not (hover: hover) {
    .btn {
      &:hover, &:focus, &:active {
        background-color: transparent;
      }
    }
  }
</style>