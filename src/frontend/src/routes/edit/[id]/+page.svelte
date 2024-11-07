<script lang="ts">
	import type { PageData } from './$types';
	import LineMdAlert from '~icons/line-md/alert';
	import { goto } from '$app/navigation';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import type { FlyParams } from 'svelte/transition';
	import { fly } from 'svelte/transition';
	import { cubicIn } from 'svelte/easing';
	import LineMdRemove from '~icons/line-md/remove';
	import { deserialize } from '$app/forms';
	import type { ActionResult } from '@sveltejs/kit';

	export let data: PageData;

	const baseDetailsPage = `/details/${data.info.shortened}`;

	let name: string = data.info.name;
	let original: string = data.info.original;
	let shortened: string = data.info.shortened;

	function fillFieldsIfEmpty() {
		if (!name) {
			name = data.info.name;
		}
		if (!original) {
			original = data.info.original;
		}
		if (!shortened) {
			shortened = data.info.shortened;
		}
	}

	function addProtocolIfNeeded() {
		// use regex to check if the url has a protocol
		if (!/^https?:\/\//i.test(original)) {
			original = `https://${original}`;
		}
	}

	function trimFields() {
		name = name.trim();
		original = original.trim();
		shortened = shortened.trim();
	}

	let isSaving = false;
	let isError = false;
	let errorDescription = '';

	async function handleFormSubmission(event: { currentTarget: EventTarget & HTMLFormElement}) {
		isError = false;
		isSaving = true;
		fillFieldsIfEmpty();
		trimFields();
		addProtocolIfNeeded();

		const payload = {
			current_shortened: data.info.shortened,
			name,
			original,
			shortened
		};

		const response = await fetch(event.currentTarget.action, {
			method: 'POST',
			body: JSON.stringify(payload),
		});

		const result: ActionResult = deserialize(await response.text());

		if (result.type === 'failure') {
			// rerun all `load` functions, following the successful update
			console.error('Error saving data');
			isError = true;
			if (result.data) {
				errorDescription = result.data.description;
			}
		}

		// applyAction(result);

		isSaving = false;

		await goto(`/details/${shortened}`);
	}

	const flyInOptions: FlyParams = {
		delay: 0,
		duration: 300,
		easing: cubicIn,
		x: '-25%'
	};
</script>

<main class="mt-3">
	<div class="w-90 mx-auto">
		<h1 class="mb-0 d-flex flex-1 justify-content-center align-items-center responsive-title-size">
			Editing <a class="ms-2 text-decoration-none" href={baseDetailsPage}> {data.info.name}</a>
		</h1>

		<hr class="my-3" />

		<form on:submit|preventDefault={handleFormSubmission}>
			<div class="container">
				<div class="row g-4">
					<div class="col-12">
						<label class="responsive-label-size" for="name">Name</label>
						<input autocomplete="off" bind:value={name} class="form-control" id="name" placeholder={data.info.name}
									 type="text" />
					</div>

					<div class="col-12">
						<label class="responsive-label-size" for="original">
							<span class="d-flex align-items-center">
								Original
								<span class="text-warning ms-2 justify-content-end d-flex align-items-center">
									<LineMdAlert class="me-1" />
									Editing this will redirect all traffic
								</span>
							</span>
						</label>
						<input bind:value={original} class="form-control" id="original" placeholder={data.info.original}
									 autocomplete="off" type="text" />
					</div>

					<div class="col-12">
						<label class="responsive-label-size" for="shortened">
							<span class="d-flex align-items-center">
								Shortened
								<span class="text-warning ms-2 justify-content-end d-flex align-items-center">
									<LineMdAlert class="me-1" />
									Editing this will make the previous url invalid
								</span>
							</span>
						</label>
						<input bind:value={shortened} class="form-control" id="shortened" placeholder={data.info.shortened}
									 autocomplete="off" type="text" />
					</div>

					<div class="col-12">
						<div class="d-flex justify-content-center mt-3">
							<button class="btn btn-primary" type="submit">
								{#if isSaving}
									<span class="d-flex align-items-center" in:fly={flyInOptions}>
										<LineMdLoadingLoop class="ms-1" />
										Saving...
									</span>
								{:else}
									Save
								{/if}
							</button>
							<a class="btn btn-outline-secondary ms-2" href={baseDetailsPage}>
								Cancel
							</a>
						</div>

						{#if isError}
							<div class="alert alert-danger mt-3 d-flex justify-content-center align-items-center mt-5" role="alert">
								<LineMdRemove class="me-2" />
								Error saving data: {errorDescription}
							</div>
						{/if}
					</div>
				</div>
			</div>
		</form>
	</div>
</main>

<style>
    .responsive-title-size {
        font-size: calc(0.7em + 1vw);
    }

    .responsive-label-size {
        font-size: calc(0.4em + 0.7vw);
    }
</style>