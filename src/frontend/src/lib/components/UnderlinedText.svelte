<script lang="ts">
	import tinycolor from 'tinycolor2';

	export let color: string;

	const lighteningLevel = 20;

	const lightenedColor = tinycolor(color).lighten(lighteningLevel).toString();
</script>

<span class="custom-underline fw-bold" style={`text-decoration-color: ${color}; --lightened-color: ${lightenedColor}`}>
	<slot />
</span>

<style>
    .custom-underline {
        text-decoration: underline;
        text-underline-offset: 2px;
    }

    .custom-underline {
        color: var(--lightened-color);
    }
</style>