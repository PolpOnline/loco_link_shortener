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
	import { type FlyParams, slide } from 'svelte/transition';
	import { cubicIn } from 'svelte/easing';
	import { flip, type FlipParams } from 'svelte/animate';
	import LineMdClipboardArrow from '~icons/line-md/clipboard-arrow';
	import LineMdConfirm from '~icons/line-md/confirm';


	export let data: PageData;

	let fullShortened = `${base}/x/${data.info.shortened}`;
	let fullShortenedView = fullShortened.replace(/^(?:https?:\/\/)?(?:www\.)?/i, '');
	let fullOriginal = data.info.original.replace(/^(?:https?:\/\/)?(?:www\.)?/i, '');

	async function deleteUrl() {
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

	const slideInOptions: FlyParams = {
		y: '-50%',
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
		<div class="d-flex align-items-center link-header">
			<button class="btn btn-outline-secondary me-auto" on:click={async () => await goto('/')}>
				<HeroiconsArrowLeft />
			</button>

			<h1 class="mb-0 d-flex flex-1 justify-content-center info-name">
				{data.info.name}
			</h1>

			<button class="btn btn-outline-primary ms-auto" on:click={refresh}>
				<span class="d-inline-block" class:rotating={isRefreshing}>
					<TablerRefresh />
				</span>
			</button>
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
				<span class="d-flex align-items-center">
					<HeroiconsTrash class="me-2" />
					Delete this URL
				</span>
			</button>
		</div>
	</div>
</main>

<style lang="scss">
  // Fix for really small screens, causing the title to go on two lines
  @media screen and (max-width: 430px) {
    .info-name {
      font-size: 6vw;
    }
  }

  .flex-1 {
    flex: 1;
  }

  @keyframes rotating {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .rotating {
    animation: rotating 2s linear infinite;
  }
</style>