<script lang="ts">
	import FormRow from '$lib/components/FormRow.svelte';
	import { client } from '$lib/graphql/client';
	import type { ClientError } from 'graphql-request';

	let email = '';

	let errors: string | undefined = undefined;

	let loading = false;

	let submitted = false;
	const onSubmit = (e: Event) => {
		e.preventDefault();

		loading = true;
		errors = undefined;
		client
			.requestResetToken({ email })
			.then(() => {
				submitted = true;
				loading = false;
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	$: canSave = email != '' && submitted == false;
</script>

<svelte:head>
	<title>DiveDB - Reset Password</title>
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<h1 class="page-title">Reset your password</h1>
		</div>
	</div>
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<p>Type your email below to have a password reset email sent</p>
		</div>
	</div>
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<form class="form-horizontal" on:submit={onSubmit}>
				<FormRow name="Email">
					<input type="text" placeholder="Email" bind:value={email} class="form-input" />
				</FormRow>
				<FormRow name="">
					<button class="btn btn-primary" type="submit" disabled={canSave == false}
						>Reset Password</button
					>
				</FormRow>
			</form>
		</div>
		{#if errors}
			<div class="toast">{errors}</div>
		{/if}
		{#if loading}
			<div class="column col-12">
				<div class="loading loading-lg"></div>
			</div>
		{/if}
		{#if submitted}
			<div class="column col-12">
				<strong> Reset Submitted! Please check your email for further instructions </strong>
			</div>
		{/if}
	</div>
</div>
