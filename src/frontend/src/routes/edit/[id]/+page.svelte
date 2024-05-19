<script lang="ts">
	import type { PageData } from './$types';
	import LineMdAlert from '~icons/line-md/alert';
	import { goto } from '$app/navigation';
	import { send } from '$lib/api';
	import { get as storeGet } from 'svelte/store';
	import { jwt } from '$lib/stores/auth';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import type { FlyParams } from 'svelte/transition';
	import { fly } from 'svelte/transition';
	import { cubicIn } from 'svelte/easing';
	import LineMdRemove from '~icons/line-md/remove';

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

	let isSaving = false;
	let isError = false;

	async function saveForm() {
		isError = false;
		isSaving = true;
		fillFieldsIfEmpty();

		const payload = {
			current_shortened: data.info.shortened,
			name,
			original,
			shortened
		};

		let res: Response = await send({
			method: 'PUT',
			path: 'edit',
			data: payload,
			token: storeGet(jwt)
		});

		isSaving = false;

		if (res.status !== 200) {
			console.error('Error saving data');
			isError = true;
			return;
		}

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
		<h1 class="mb-0 d-flex flex-1 justify-content-center align-items-center info-name">
			Editing <a class="ms-2 text-decoration-none" href={baseDetailsPage}> {data.info.name}</a>
		</h1>

		<hr class="my-3" />

		<div class="container">
			<div class="row g-4">
				<div class="col-12">
					<label for="name">Name</label>
					<input bind:value={name} class="form-control" id="name" placeholder={data.info.name} type="text" />
				</div>

				<div class="col-12">
					<label for="original">
							<span class="d-flex align-items-center">
								Original
								<span class="text-warning ms-2 justify-content-end d-flex align-items-center">
									<LineMdAlert class="me-1" />
									Editing this will redirect all traffic
								</span>
							</span>
					</label>
					<input bind:value={original} class="form-control" id="original" placeholder={data.info.original}
								 type="text" />
				</div>

				<div class="col-12">
					<label for="shortened">
							<span class="d-flex align-items-center">
								Shortened
								<span class="text-warning ms-2 justify-content-end d-flex align-items-center">
									<LineMdAlert class="me-1" />
									Editing this will make the previous url invalid
								</span>
							</span>
					</label>
					<input bind:value={shortened} class="form-control" id="shortened" placeholder={data.info.shortened}
								 type="text" />
				</div>
			</div>
		</div>

		<div class="d-flex justify-content-center mt-3">
			<button class="btn btn-primary" on:click={saveForm}>
				{#if isSaving}
					<span class="d-flex align-items-center" in:fly={flyInOptions}>
						<LineMdLoadingLoop class="ms-1" />
						Saving...
					</span>
				{:else}
					Save
				{/if}
			</button>
			<button class="btn btn-outline-secondary ms-2" on:click={async () => await goto(baseDetailsPage)}>Cancel</button>
		</div>

		{#if isError}
			<div class="alert alert-danger mt-3 d-flex justify-content-center align-items-center" role="alert">
				<LineMdRemove class="me-2" />
				Error saving data
			</div>
		{/if}
	</div>
</main>