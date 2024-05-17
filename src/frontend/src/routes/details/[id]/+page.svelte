<script lang="ts">
	import { base, send } from '$lib/api';
	import type { DeleteRequest, InfoLinkView } from '$lib/models';
	import { get as storeGet } from 'svelte/store';
	import { jwt } from '$lib/stores/auth';
	import HeroiconsTrash from '~icons/heroicons/trash';
	import HeroiconsClock from '~icons/heroicons/clock';
	import HeroiconsArrowLeft from '~icons/heroicons/arrow-left';
	import IconoirIpAddressTag from '~icons/iconoir/ip-address-tag';
	import LucideExpand from '~icons/lucide/expand';
	import LucideShrink from '~icons/lucide/shrink';
	import MdiAnonymous from '~icons/mdi/anonymous';
	import HeroiconsCalendar from '~icons/heroicons/calendar';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';

	export let data: PageData;

	// @ts-ignore
	const info: InfoLinkView = data.info;

	let fullShortened = `${base}/x/${info.shortened}`.replace(/^(?:https?:\/\/)?(?:www\.)?/i, '');
	let fullOriginal = info.original.replace(/^(?:https?:\/\/)?(?:www\.)?/i, '');

	async function deleteUrl() {
		try {
			let payload: DeleteRequest = {
				shortened: info.shortened
			};

			await send({
				method: 'DELETE',
				path: `delete`,
				data: payload,
				token: storeGet(jwt)
			});
		} catch (error) {
			console.error(error);
			throw Error('Failed to delete url');
		}

		await goto('/');
	}
</script>

<main>
	<div class="w-90 mx-auto">
		<div class="d-flex align-items-center link-header">
			<button class="btn btn-outline-secondary me-auto" on:click={async () => await goto('/')}>
				<HeroiconsArrowLeft />
			</button>

			<h1 class="mb-0 d-flex info-name flex-1 justify-content-center">
				{info.name}
			</h1>
		</div>

		<hr class="my-3" />

		<div class="d-flex align-items-center mt-3">
			<LucideExpand class="me-2" />
			<span class="fw-bold me-1">
				Original:
			</span>
			<a href={info.original}>{fullOriginal}</a>
		</div>

		<div class="d-flex align-items-center mt-3">
			<LucideShrink class="me-2" />
			<span class="fw-bold me-1">
				Shortened:
			</span>
			<a href={fullShortened}>{fullShortened}</a>
		</div>

		<div class="d-flex align-items-center mt-3">
			<HeroiconsCalendar class="me-2" />
			<span class="fw-bold me-1">
				Created:
			</span>
			{new Date(info.created_at).toLocaleString()}
		</div>

		<div class="table-responsive mt-3">
			<table class="table caption-top rounded">
				<caption>
					{#if info.clicks.length === 1}
						Displaying {info.clicks.length} click
					{:else if info.clicks.length > 1}
						Displaying {info.clicks.length} clicks
					{:else}
						No clicks yet
					{/if}
				</caption>
				<thead>
					<tr>
						<th scope="col">
							<div class="d-flex align-items-center">
								<HeroiconsClock class="me-2" />
								Clicked
							</div>
						</th>
						<th scope="col">
							<div class="d-flex align-items-center">
								<IconoirIpAddressTag class="me-2" />
								Address
							</div>
						</th>
						<th scope="col">
							<div class="d-flex align-items-center">
								<MdiAnonymous class="me-2" />
								User Agent
							</div>
						</th>
					</tr>
				</thead>
				<tbody>
					{#if info.clicks.length === 0}
						<tr>
							<td colspan="3" class="text-center">Clicks will appear here!</td>
						</tr>
					{/if}

					{#each info.clicks as click (click.clicked_at)}
						<tr>
							<td>{new Date(click.clicked_at).toLocaleString()}</td>
							<td>{click.address}</td>
							<td>
								{#if click.user_agent}
									{click.user_agent}
								{:else}
									Unknown
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<button class="btn btn-outline-danger mt-5" on:click={deleteUrl}>
			<span class="d-flex align-items-center">
				<HeroiconsTrash class="me-2" />
				Delete this URL
			</span>
		</button>
	</div>
</main>

<style lang="scss">
  .flex-1 {
    flex: 1;
  }
</style>