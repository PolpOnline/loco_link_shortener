<script lang="ts">
	export let color: string;
</script>

<span class="custom-underline" style={`text-decoration-color: ${color};`}>
	<slot />
</span>

<style>
    .custom-underline {
        text-decoration: underline;
        text-underline-offset: 2px;
    }
</style>