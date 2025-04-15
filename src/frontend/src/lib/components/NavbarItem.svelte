<script lang="ts">
	import { page } from '$app/stores';

	interface Props {
		href: string;
		text: string;
	}

	let { href, text }: Props = $props();
</script>

<li class="nav-item">
	<a
		aria-current={$page.url.pathname === href ? 'page' : undefined}
		class="nav-link"
		class:active={$page.url.pathname === href}
		{href}
	>
		{text}
	</a>
</li>
