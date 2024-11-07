<svelte:head>
	<title>Login</title>
</svelte:head>

<script lang="ts">
	import FlatColorIconsGoogle from '~icons/flat-color-icons/google';

	export let data;
</script>

<div class="d-flex flex-column justify-content-center align-items-center vh-100">
	<a class="btn btn-outline-primary d-flex align-items-center" href={data.gotoUrl}>
		<FlatColorIconsGoogle />
		<span class="ms-2">Login with Google</span>
	</a>
</div>


<style lang="scss">
  .btn {
    &:hover, &:focus, &:active {
      background-color: transparent;
    }
  }
</style>