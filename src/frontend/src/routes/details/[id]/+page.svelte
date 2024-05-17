<script lang="ts">
	import { base, send } from '$lib/api';
	import type { DeleteRequest, InfoLinkView } from '$lib/models';
	import { get as storeGet } from 'svelte/store';
	import { jwt } from '$lib/stores/auth';
	import HeroiconsTrash from '~icons/heroicons/trash';
	import HeroiconsArrowRightCircle from '~icons/heroicons/arrow-right-circle';
	import HeroiconsClock from '~icons/heroicons/clock';
	import HeroiconsArrowLeft from '~icons/heroicons/arrow-left';
	import IconoirIpAddressTag from '~icons/iconoir/ip-address-tag';
	import MaterialSymbolsContentCopyOutline from '~icons/material-symbols/content-copy-outline';
	import MdiAnonymous from '~icons/mdi/anonymous';
	import type { PageData } from './$types';

	export let data: PageData;

	const info: InfoLinkView = data.info;

	let fullShortened = `${base}/x/${info.shortened}`;

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

		window.location.href = '/';
	}
</script>

<main>
	<div class="w-90 mx-auto">
		<div class="d-flex align-items-center link-header">
			<button class="btn btn-outline-secondary me-auto" on:click={() => window.location.href = '/'}>
				<HeroiconsArrowLeft />
			</button>

			<h1 class="mb-0 d-flex info-name flex-1 justify-content-center">
				{info.name}
			</h1>
		</div>

		<hr class="my-3" />

		<div class="mt-3">
			<div class="d-inline-flex align-items-center">
				<MaterialSymbolsContentCopyOutline class="me-1" />
				<a href={fullShortened}>{fullShortened}</a>
			</div>
			<div class="d-inline-flex align-items-center">
				<HeroiconsArrowRightCircle class="me-1" />
				<MaterialSymbolsContentCopyOutline class="me-1" />
				<a href={info.original}> {info.original} </a>
			</div>
		</div>

		<div class="d-flex align-items-center mt-3">
			<HeroiconsClock class="me-2" />
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

					{#each info.clicks as click}
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