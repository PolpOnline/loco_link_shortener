<script lang="ts">
	import FlatColorIconsGoogle from '~icons/flat-color-icons/google';
	import { get } from '$lib/api';

	let href = '';
	$ : {
		Promise.resolve(get('oauth2/google')
			.then(res => res.text())
			.then((x) => href = x)
			.catch(err => console.error(err)));
	}
	// async function getUrl() {
	// 	const res = await get('oauth2/google');
	//
	// 	console.log(res.text());
	//
	// 	if (!res.ok) {
	// 		throw Error(res.text());
	// 	}
	//
	// 	url = await res.text();
	// }
</script>

<div class="d-flex flex-column justify-content-center align-items-center vh-100">
	<a href={href}>
		<button class="btn btn-primary d-flex align-items-center">
			<FlatColorIconsGoogle />
			<span class="ms-2">Login with Google</span>
		</button>
	</a>
</div>
