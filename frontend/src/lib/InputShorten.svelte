<script lang="ts">
	import { slide } from 'svelte/transition';

	import HeroiconsCheck from '~icons/heroicons/check';
	import HeroiconsClipboard from '~icons/heroicons/clipboard';

	const urlRegex = new RegExp('https?:\\/\\/(www\\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\\.[a-zA-Z0-9()]{1,6}\\b([-a-zA-Z0-9()@:%_\+.~#?&/=]*)');
	let url = '';
	let invalidForm = false;
	let invalidFeedback = '';

	function checkIsValid() {
		invalidForm = !urlRegex.test(url);
		invalidFeedback = invalidForm ? 'Please insert a valid url' : '';
	}

	let shortenedUrl = '';

	shortenedUrl = 'a';


	let isCheckMarkDisplayed = false;

	function copyShortened() {
		navigator.clipboard.writeText(shortenedUrl);
		isCheckMarkDisplayed = true;
		setTimeout(() => {
			isCheckMarkDisplayed = false;
		}, 1200);
	}
</script>

<!--convert to vanilla bootstrap-->

<div class="container">
	<div class="row">
		<div class="col-md-10 col-12">
			<input bind:value={url} class="form-control" placeholder="Insert your link here" type="url" />
		</div>
		<div class="col-md-2 col-12 mt-2 mt-md-0">
			<button class="btn btn-primary w-100" disabled={!url} on:click={checkIsValid}>Shorten</button>
		</div>
	</div>
	<!--	test purposes -->
	<!--		<button on:click={() => {-->
	<!--			if (shortenedUrl) {-->
	<!--				shortenedUrl = ''-->
	<!--			} else { -->
	<!--				shortenedUrl = 'a'-->
	<!--			}-->
	<!--		}}></button>-->
	{#if shortenedUrl}
		<div transition:slide>
			<div class="row mt-2">
				<div class="col-12">
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