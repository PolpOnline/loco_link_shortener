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
		<button class="btn btn-primary" on:click={() => window.location.href = '/'}>
			<HeroiconsArrowLeft class="w-6 h-6" />
			Go back
		</button>

		<h1 class="mt-3">{info.name}</h1>

		<p>
			<a href={fullShortened}>{fullShortened}</a>
			<HeroiconsArrowRightCircle />
			<a href={info.original}> {info.original} </a></p>

		<p>
			<HeroiconsClock />
			{info.created_at.toLocaleString()}
		</p>

		<p>Displaying {info.clicks.length} clicks</p>

		<!--{#each data.clicks as click}-->
		<!--	<p>{click.clicked_at} - {click.address} - {click.user_agent}</p>-->
		<!--{/each}-->
		<div class="table-responsive">
			<table class="table">
				<thead>
					<tr>
						<th scope="col">
							<div class="d-flex align-items-center">
								<HeroiconsClock class="me-2" />
								Clicked at
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
					{#each info.clicks as click}
						<tr>
							<td>{click.clicked_at.toLocaleString()}</td>
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

		<button class="btn btn-danger" on:click={deleteUrl}>
			<HeroiconsTrash class="w-6 h-6" />
			Delete this URL
		</button>
	</div>
</main>
