<svelte:head>
	<title>Details for {data.info.name}</title>
</svelte:head>

<script lang="ts">
	import { base, send } from '$lib/api';
	import type { DeleteRequest } from '$lib/models';
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
	import { goto, invalidateAll } from '$app/navigation';
	import LineMdAlert from '~icons/line-md/alert';
	import TablerRefresh from '~icons/tabler/refresh';
	import { slide, type SlideParams } from 'svelte/transition';
	import { cubicIn } from 'svelte/easing';
	import { flip, type FlipParams } from 'svelte/animate';
	import LineMdClipboardArrow from '~icons/line-md/clipboard-arrow';
	import LineMdConfirm from '~icons/line-md/confirm';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import LineMdEdit from '~icons/line-md/edit';


	export let data: PageData;

	let fullShortened = `${base}/x/${data.info.shortened}`;
	let fullShortenedView = fullShortened.replace(/^(?:https?:\/\/)?(?:www\.)?/i, '');
	let fullOriginal = data.info.original.replace(/^(?:https?:\/\/)?(?:www\.)?/i, '');

	let isDeleting = false;

	async function deleteUrl() {
		isDeleting = true;
		try {
			let payload: DeleteRequest = {
				shortened: data.info.shortened
			};

			await send({
				method: 'DELETE',
				path: `delete`,
				data: payload,
				token: storeGet(jwt)
			});
		} catch (error) {
			console.error(error);
		}

		await goto('/');
	}

	let isRefreshing = false;

	async function refresh() {
		if (isRefreshing) return;
		isRefreshing = true;
		await invalidateAll();
		isRefreshing = false;
	}

	const slideInOptions: SlideParams = {
		axis: 'y',
		duration: 300,
		easing: cubicIn
	};

	const flipOptions: FlipParams = {
		duration: 300
	};

	let isCheckMarkDisplayed = false;

	function copyShortened() {
		navigator.clipboard.writeText(fullShortened);
		isCheckMarkDisplayed = true;
		setTimeout(() => {
			isCheckMarkDisplayed = false;
		}, 1200);
	}
</script>

<main class="mt-3">
	<div class="w-90 mx-auto">
		<div class="d-flex align-items-center">
			<a class="btn btn-outline-secondary me-auto" href="/">
				<HeroiconsArrowLeft />
			</a>
			<h1 class="mb-0 d-flex flex-1 justify-content-center responsive-title-size">
				{data.info.name}
			</h1>
			<a class="btn btn-outline-primary ms-auto" href={`/edit/${data.info.shortened}`}>
				<LineMdEdit />
			</a>
		</div>

		<hr class="my-3" />

		<div class="d-flex align-items-center my-3">
			<HeroiconsCalendar class="me-2" />
			<span class="fw-bold me-1">
				Created:
			</span>
			{new Date(data.info.created_at).toLocaleString()}
		</div>

		<div class="d-flex align-items-center my-3">
			<LucideExpand class="me-2" />
			<span class="fw-bold me-1">
				Original:
			</span>
			<a href={data.info.original} target="_blank">{fullOriginal}</a>
		</div>

		<div class="d-flex align-items-center my-3">
			<LucideShrink class="me-2" />
			<span class="fw-bold me-1">
				Shortened:
			</span>
			<a href={fullShortened} target="_blank">{fullShortenedView}</a>
		</div>

		<button class="btn btn-outline-secondary" on:click={copyShortened}>
			{#if isCheckMarkDisplayed}
				<span class="d-flex align-items-center">
					<LineMdConfirm class="me-2" />
					Copied!
				</span>
			{:else}
			<span class="d-flex align-items-center">
					<LineMdClipboardArrow class="me-2" />
					Copy
				</span>
			{/if}
		</button>

		<button class="btn btn-outline-primary" on:click={refresh}>
				<span class="d-flex align-items-center">
					<span class="d-inline-block me-1" class:rotating={isRefreshing}>
						<TablerRefresh />
					</span>
					Refresh table
				</span>
		</button>

		<div class="table-responsive my-3">
			<table class="table caption-top rounded">
				<caption>
					{#if data.info.clicks.length === 1}
						Displaying {data.info.clicks.length} click
					{:else if data.info.clicks.length > 1}
						Displaying {data.info.clicks.length} clicks
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
					{#if data.info.clicks.length === 0}
						<tr>
							<td colspan="3" class="text-center">Clicks will appear here!</td>
						</tr>
					{/if}

					{#each data.info.clicks as click (click.clicked_at)}
						<tr in:slide={slideInOptions} animate:flip={flipOptions}>
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


		<div class="mt-5 text-center">
			<p class="d-flex align-items-center fw-bold text-danger justify-content-center mb-1">
				<LineMdAlert class="me-2" />
				Danger
			</p>

			<button class="btn btn-outline-danger" on:click={deleteUrl}>
				{#if isDeleting}
					<span class="d-flex align-items-center">
						<LineMdLoadingLoop class="me-2" />
						Deleting...
					</span>
				{:else}
					<span class="d-flex align-items-center">
						<HeroiconsTrash class="me-2" />
						Delete this URL
					</span>
				{/if}
			</button>
		</div>
	</div>
</main>

<style>
    .responsive-title-size {
        font-size: calc(0.6em + 1vw);
    }
</style>