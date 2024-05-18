<script lang="ts">
	import { slide } from 'svelte/transition';
	import HeroiconsLink from '~icons/heroicons/link';
	import HeroiconsPencilSquareSolid from '~icons/heroicons/pencil-square-solid';
	import GgShortcut from '~icons/gg/shortcut';
	import { base, send } from '$lib/api';
	import type { AddRequest, AddResponse } from '$lib/models';
	import MaterialSymbolsKeyboardArrowDownRounded from '~icons/material-symbols/keyboard-arrow-down-rounded';
	import MaterialSymbolsKeyboardArrowUpRounded from '~icons/material-symbols/keyboard-arrow-up-rounded';
	import { jwt } from '$lib/stores/auth';
	import { get as storeGet } from 'svelte/store';
	import { invalidateAll } from '$app/navigation';
	import LineMdClipboardArrow from '~icons/line-md/clipboard-arrow';
	import LineMdConfirm from '~icons/line-md/confirm';

	const urlRegex = new RegExp('https?:\\/\\/(www\\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\\.[a-zA-Z0-9()]{1,6}\\b([-a-zA-Z0-9()@:%_\+.~#?&/=]*)');
	let url = '';
	let customName = '';
	let customShortened = '';
	let invalidForm = false;
	let invalidFeedback = '';
	let shortenedUrl = '';
	let isAdvancedOpen = false;

	function checkIsValid() {
		invalidForm = !urlRegex.test(url);
		invalidFeedback = invalidForm ? 'Please insert a valid url' : '';
	}

	$: fullShortened = `${base}/x/${shortenedUrl}`;

	let isCheckMarkDisplayed = false;

	function copyShortened() {
		navigator.clipboard.writeText(fullShortened);
		isCheckMarkDisplayed = true;
		setTimeout(() => {
			isCheckMarkDisplayed = false;
		}, 1200);
	}

	async function submitForm() {
		checkIsValid();

		if (invalidForm) {
			return Error(invalidFeedback);
		}

		let payload: AddRequest = {
			url
		};

		if (customName) {
			payload.name = customName;
		}

		if (customShortened) {
			payload.custom = customShortened;
		}

		let response: AddResponse = await send({
			method: 'POST',
			path: 'add',
			data: payload,
			token: storeGet(jwt)
		});

		shortenedUrl = response.shortened;

		await invalidateAll();
	}

	function handleKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			submitForm();
		}
	}
</script>


<div class="container px-0">
	<!-- input part -->
	<div class="row">
		<div class="col-md-10 col-12">
			<div class="d-flex align-items-center text-body">
				<HeroiconsLink class="me-2" />
				<input bind:value={url} class="form-control" on:keydown={handleKeyDown} placeholder="Insert your link here"
							 type="url" />
			</div>
		</div>
		<div class="col-md-2 col-12 mt-2 mt-md-0">
			<button class="btn btn-primary w-100" disabled={!url} on:click={submitForm} on:keydown={handleKeyDown}>Shorten
			</button>
		</div>
	</div>
	<!--	advanced options part-->
	<div class="row mt-2">
		<div class="col-12">
			<button class="btn btn-outline-primary w-100" on:click={() => isAdvancedOpen = !isAdvancedOpen}>
				{#if isAdvancedOpen}
					<MaterialSymbolsKeyboardArrowUpRounded />
					Advanced
				{:else}
					<MaterialSymbolsKeyboardArrowDownRounded />
					Advanced
				{/if}
			</button>
		</div>
	</div>
	{#if isAdvancedOpen}
		<div class="row" transition:slide>
			<div class="col-md-6 col-12 mt-2">
				<div class="d-flex align-items-center text-body">
					<HeroiconsPencilSquareSolid class="me-2" />
					<input bind:value={customName} class="form-control" placeholder="Custom name" type="text" />
				</div>
			</div>
			<div class="col-md-6 col-12 mt-2">
				<div class="d-flex align-items-center text-body">
					<GgShortcut class="me-2" />
					<input bind:value={customShortened} class="form-control" placeholder="Custom shortened" type="text" />
				</div>
			</div>
		</div>
	{/if}

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
					<input disabled={true} type="text" class="form-control" value={fullShortened} />
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