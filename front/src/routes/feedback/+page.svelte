<script lang="ts">
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	import { client } from '$lib/graphql/client';
	import type { ClientError } from 'graphql-request';

	let feedback = '';

	$: canSave = feedback != '';

	let errors: string | undefined = undefined;

	const onSubmit = () => {
		loading = true;
		client
			.addFeedback({ feedback })
			.then(() => {
				loading = false;
				submitted = true;
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	let submitted = false;

	let loading = false;
</script>

<CheckLogin />

<div class="container grid-lg">
	<div class="columns">
		<div class="column">
			<h1 class="page-title">Add Feedback</h1>
		</div>

		{#if submitted}
			<div class="column col-12 col-sm-12">Submitted! Thanks for your feedback</div>
		{:else}
			<div class="column col-12 col-sm-12">
				<textarea
					placeholder="Enter in any feedback you may have for DiveDB, including any changes you'd like to see or challenges you are having."
					bind:value={feedback}
					rows="8"
					class="form-input"
				/>
			</div>
			<div class="column col-12 col-sm-12 submit-button">
				<button on:click={onSubmit} disabled={canSave == false} class="btn btn-primary">
					Submit
				</button>
			</div>
		{/if}
		{#if errors}
			<div class="column col-12">
				<div class="toast">{errors}</div>
			</div>
		{/if}
		{#if loading}
			<div class="column col-12">
				<div class="loading loading-lg" />
			</div>
		{/if}
	</div>
</div>

<style lang="scss">
	.submit-button {
		margin-top: 0.5rem;
	}
</style>
