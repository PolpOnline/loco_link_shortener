<script lang="ts">
    import FlatColorIconsGoogle from '~icons/flat-color-icons/google';
    import {get} from '$lib/api';

    // let href = '';
    // $ : {
    // 	Promise.resolve(get('oauth2/google')
    // 		.then(res => res.text())
    // 		.then((x) => href = x)
    // 		.catch(err => console.error(err)));
    // }
    async function redirect() {
        const res = await fetch("http://localhost:3000/api/oauth2/google", {
            credentials:"include"
        })

        if (!res.ok) {
            throw Error(res.text());
        }

        // window.location.href = await res.text();
    }
</script>

<div class="d-flex flex-column justify-content-center align-items-center vh-100">
    <button class="btn btn-primary d-flex align-items-center" on:click={redirect}>
        <FlatColorIconsGoogle/>
        <span class="ms-2">Login with Google</span>
    </button>
</div>
