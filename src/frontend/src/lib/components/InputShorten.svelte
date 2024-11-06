<script lang="ts">
	import { fly, type FlyParams, slide } from 'svelte/transition';
	import HeroiconsLink from '~icons/heroicons/link';
	import HeroiconsPencilSquareSolid from '~icons/heroicons/pencil-square-solid';
	import GgShortcut from '~icons/gg/shortcut';
	import { API_URL } from '$lib/api';
	import type { AddRequest, AddResponse } from '$lib/models';
	import MaterialSymbolsKeyboardArrowDownRounded from '~icons/material-symbols/keyboard-arrow-down-rounded';
	import MaterialSymbolsKeyboardArrowUpRounded from '~icons/material-symbols/keyboard-arrow-up-rounded';
	import { invalidateAll } from '$app/navigation';
	import LineMdClipboardArrow from '~icons/line-md/clipboard-arrow';
	import LineMdConfirm from '~icons/line-md/confirm';
	import SvgSpinners90Ring from '~icons/svg-spinners/90-ring';
	import { cubicIn } from 'svelte/easing';

	const urlRegex = new RegExp('https?:\\/\\/(www\\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\\.[a-zA-Z0-9()]{1,6}\\b([-a-zA-Z0-9()@:%_\+.~#?&/=]*)');
	let url = '';
	let customName = '';
	let customShortened = '';
	let invalidForm = false;
	let invalidFeedback = '';
	let shortenedUrl = '';
	let isOptionsOpen = false;
	let isShortening = false;

	function addProtocolIfNeeded() {
		// use regex to check if the url has a protocol
		if (!/^https?:\/\//i.test(url)) {
			url = `https://${url}`;
		}
	}

	function checkIsValid() {
		invalidForm = !urlRegex.test(url);
		invalidFeedback = invalidForm ? 'Please insert a valid url' : '';
	}

	$: fullShortened = `${API_URL}/x/${shortenedUrl}`;

	let isCheckMarkDisplayed = false;

	function copyShortened() {
		navigator.clipboard.writeText(fullShortened);
		isCheckMarkDisplayed = true;
		setTimeout(() => {
			isCheckMarkDisplayed = false;
		}, 1200);
	}

	function trimFields() {
		url = url.trim();
		customName = customName.trim();
		customShortened = customShortened.trim();
	}

	async function handleFormSubmission(event: { currentTarget: EventTarget & HTMLFormElement }) {
		invalidForm = false;
		invalidFeedback = '';

		isShortening = true;

		trimFields();
		addProtocolIfNeeded();
		checkIsValid();

		if (invalidForm) {
			isShortening = false;
			invalidForm = true;
			invalidFeedback = 'Please insert a valid url';
			return;
		}

		const payload: AddRequest = {
			url,
			name: customName || undefined,
			custom: customShortened || undefined
		};

		const res = await fetch('/api/add', {
			method: 'POST',
			body: JSON.stringify(payload),
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (res.status !== 200) {
			invalidForm = true;
			invalidFeedback = await res.text();
			isShortening = false;
			return;
		}

		const response: AddResponse = await res.json();

		shortenedUrl = response.shortened;

		await invalidateAll();

		isShortening = false;

		// Re-invalidate the cache again after 1 second (to load images)
		setTimeout(() => {
			invalidateAll();
		}, 1000);
	}

	// Shorten transition, fly from left to right when in, fade out when out
	const flyInOptions: FlyParams = {
		delay: 0,
		duration: 300,
		easing: cubicIn,
		x: '-25%'
	};
</script>


<div class="container px-0">
	<!-- input part -->
	<form on:submit|preventDefault={handleFormSubmission}>
		<div class="row">
			<div class="col-md-10 col-12">
				<div class="d-flex align-items-center text-body">
					<HeroiconsLink class="me-2" />
					<div class="input-group has-validation">
						<input bind:value={url} class="form-control" class:is-invalid={invalidForm} id="invalidationUrl"
									 autocomplete="off" placeholder="Insert your link here" type="text"
						/>
						{#if invalidForm}
							<div class="invalid-feedback">
								{invalidFeedback}
							</div>
						{/if}
					</div>
				</div>
			</div>
			<div class="col-md-2 col-12 mt-2 mt-md-0">
				<button class="btn btn-primary w-100" disabled={!url} type="submit">
					{#if isShortening}
						<span class="d-flex align-items-center justify-content-center" in:fly={flyInOptions}>
							<SvgSpinners90Ring class="me-2" />
							Shortening...
						</span>
					{:else}
					<span in:fly={flyInOptions}>
						Shorten
					</span>
					{/if}
				</button>
			</div>
		</div>
		<!--	advanced options part-->
		<div class="row mt-2">
			<div class="col-12">
				<button class="btn btn-outline-primary w-100" on:click={() => isOptionsOpen = !isOptionsOpen}>
					{#if isOptionsOpen}
						<MaterialSymbolsKeyboardArrowUpRounded />
					{:else}
						<MaterialSymbolsKeyboardArrowDownRounded />
					{/if}
					Options
				</button>
			</div>
		</div>
		{#if isOptionsOpen}
			<div class="row" transition:slide>
				<div class="col-md-6 col-12 mt-2">
					<div class="d-flex align-items-center text-body">
						<HeroiconsPencilSquareSolid class="me-2" />
						<input bind:value={customName} class="form-control" placeholder="Custom title" type="text"
									 id="validationCustomName" autocomplete="off" />
					</div>
				</div>
				<div class="col-md-6 col-12 mt-2">
					<div class="d-flex align-items-center text-body">
						<GgShortcut class="me-2" />
						<input bind:value={customShortened} class="form-control" placeholder="Custom shortened" type="text"
									 id="validationCustomShortened" autocomplete="off" />
					</div>
				</div>
			</div>
		{/if}
	</form>

	<!-- result part -->
	{#if shortenedUrl}
		<div transition:slide>
			<div class="row mt-2">
				<div class="col-12 text-body">
					Your shortened url is:
				</div>
			</div>
			<div class="row mt-2">
				<div class="col-md-10 col-12">
					<input disabled={true} type="text" class="form-control bg-black" value={fullShortened} />
				</div>
				<div class="col-md-2 col-12 mt-2 mt-md-0">
					<button class="btn btn-primary w-100" on:click={copyShortened}>
						{#if isCheckMarkDisplayed}
							<LineMdConfirm />
						{:else}
							<LineMdClipboardArrow />
						{/if}
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>
