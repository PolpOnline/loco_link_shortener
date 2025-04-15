<script lang="ts">
	import tinycolor from 'tinycolor2';

	interface Props {
		color: string;
		children?: import('svelte').Snippet;
	}

	let { color, children }: Props = $props();

	const lighteningLevel = 20;

	const lightenedColor = tinycolor(color).lighten(lighteningLevel).toString();
</script>

<span
	class="custom-underline fw-bold"
	style={`text-decoration-color: ${color}; --lightened-color: ${lightenedColor}`}
>
	{@render children?.()}
</span>

<style lang="scss">
	.custom-underline {
		text-decoration: underline;
		text-underline-offset: 2px;
	}

	.custom-underline {
		color: var(--lightened-color);
	}
</style>
