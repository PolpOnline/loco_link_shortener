<script lang="ts">
	import { page } from '$app/stores';

	export let href: string;
	export let text: string;
</script>

<li class="nav-item">
	<a aria-current={$page.url.pathname === href ? 'page' : undefined}
		 class="nav-link"
		 class:active={$page.url.pathname === href}
		 {href}>
		{text}
	</a>
</li>