<script lang="ts">
	import { slide } from 'svelte/transition';

	import HeroiconsCheck from '~icons/heroicons/check';
	import HeroiconsClipboard from '~icons/heroicons/clipboard';
	import { post } from '$lib/api';
	import { jwt } from '$lib/stores/auth';
	import { get as storeGet } from 'svelte/store';
	import type { AddRequest, AddResponse } from '$lib/models';

	const urlRegex = new RegExp('https?:\\/\\/(www\\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\\.[a-zA-Z0-9()]{1,6}\\b([-a-zA-Z0-9()@:%_\+.~#?&/=]*)');
	let url = '';
	let customName = '';
	let customShortened = '';
	let invalidForm = false;
	let invalidFeedback = '';
	let shortenedUrl = '';
	let isAdvancedOpen = true;

	function checkIsValid() {
		invalidForm = !urlRegex.test(url);
		invalidFeedback = invalidForm ? 'Please insert a valid url' : '';
	}

	let isCheckMarkDisplayed = false;

	function copyShortened() {
		navigator.clipboard.writeText(shortenedUrl);
		isCheckMarkDisplayed = true;
		setTimeout(() => {
			isCheckMarkDisplayed = false;
		}, 1200);
	}

	async function submitForm() {
		checkIsValid();

		if (invalidForm) {
			return;
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

		let response: AddResponse = await post('add', payload, storeGet(jwt));

		shortenedUrl = response.shortened;
	}
</script>


<div class="container">
	<!-- input part -->
	<div class="row">
		<div class="col-md-10 col-12">
			<input bind:value={url} class="form-control" placeholder="Insert your link here" type="url" />
		</div>
		<div class="col-md-2 col-12 mt-2 mt-md-0">
			<button class="btn btn-primary w-100" disabled={!url} on:click={submitForm}>Shorten</button>
		</div>
	</div>
	<!--	advanced options part-->
	<div class="row mt-2">
		<div class="col-12">
			<button class="btn btn-primary w-100" on:click={() => isAdvancedOpen = !isAdvancedOpen}>
				{#if isAdvancedOpen}
					Hide advanced options
				{:else}
					Show advanced options
				{/if}
			</button>
		</div>
	</div>
	{#if isAdvancedOpen}
		<div class="row" transition:slide>
			<div class="col-md-6 col-12 mt-2">
				<input bind:value={customName} class="form-control" placeholder="Custom name" type="text" />
			</div>
			<div class="col-md-6 col-12 mt-2">
				<input bind:value={customShortened} class="form-control" placeholder="Custom shortened" type="text" />
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
					<input disabled={true} type="text" class="form-control" value={shortenedUrl} />
				</div>
				<div class="col-md-2 col-12 mt-2 mt-md-0">
					<button class="btn btn-primary w-100" on:click={copyShortened}>
						{#if isCheckMarkDisplayed}
							<HeroiconsCheck />
						{:else}
							<HeroiconsClipboard />
						{/if}
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>