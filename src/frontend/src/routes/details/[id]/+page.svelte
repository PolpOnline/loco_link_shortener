<script lang="ts">
	import { base, del } from '$lib/api';
	import type { DeleteRequest, InfoLinkView } from '$lib/models';
	import { get as storeGet } from 'svelte/store';
	import { jwt } from '$lib/stores/auth';
	import HeroiconsTrash from '~icons/heroicons/trash';
	import HeroiconsArrowRightCircle from '~icons/heroicons/arrow-right-circle';
	import HeroiconsClock from '~icons/heroicons/clock';
	import HeroiconsArrowLeft from '~icons/heroicons/arrow-left';
	import IconoirIpAddressTag from '~icons/iconoir/ip-address-tag';
	import MdiAnonymous from '~icons/mdi/anonymous';

	let mockData: InfoLinkView = {
		name: 'hello-world',
		original: 'https://example.com',
		shortened: 'aFDSfe',
		clicks: [
			{
				clicked_at: new Date('2021-08-01T00:00:00Z'),
				address: '192.168.1.1',
				user_agent:
					'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
			},
			{
				clicked_at: new Date('2021-08-01T00:00:00Z'),
				address: '192.168.1.1'
			}
		],
		created_at: new Date('2021-08-01T00:00:00Z')
	};

	let data: InfoLinkView = mockData;

	let fullShortened = `${base}/${data.shortened}`;

	async function deleteUrl() {
		let payload: DeleteRequest = {
			shortened: data.shortened
		};

		let response = await del('delete', payload, storeGet(jwt));

		if (!response.success) {
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

		<h1 class="mt-3">{data.name}</h1>

		<p>
			<a href={fullShortened}>{fullShortened}</a>
			<HeroiconsArrowRightCircle />
			<a href={data.original}> {data.original} </a></p>

		<p>
			<HeroiconsClock />
			{data.created_at.toLocaleString()}
		</p>

		<p>Displaying {data.clicks.length} clicks</p>

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
					{#each data.clicks as click}
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
