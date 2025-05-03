<script lang="ts">
	interface Props {
		errors?: Record<string, string>;
	}

	let { errors = {} }: Props = $props();
</script>

{#if errors}
	<ul class="error-messages">
		{#each Object.keys(errors) as key (key)}
			<li>{key} {errors[key]}</li>
		{/each}
	</ul>
{/if}
