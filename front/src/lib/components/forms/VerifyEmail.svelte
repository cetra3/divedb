<script lang="ts">
	import { client } from '$lib/graphql/client';
	import type { ClientError } from 'graphql-request';

	let errors: string | undefined = $state(undefined);

	let loading = $state(false);
	let saved = $state(false);

	const onSubmit = (e: Event) => {
		e.preventDefault();

		loading = true;
		errors = undefined;
		client
			.resendVerification()
			.then(() => {
				loading = false;
				saved = true;
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	let canSave = $derived(!saved && !loading);
</script>

<div class="columns">
	<div class="column col-12 col-sm-12">
		<h3 class="page-title padding-top">Verify Email</h3>
	</div>
	<div class="column col-12 col-sm-12">
		Your email address hasn't been verified. Please check your email for a link to verify it. If you
		haven't received the email verification yet, please press the button below.
	</div>
</div>
<div class="columns">
	<div class="column col-12 col-sm-12">
		<form class="form-horizontal" onsubmit={onSubmit}>
			<button class="btn btn-primary" type="submit" disabled={canSave == false}
				>Resend Verification Email</button
			>
		</form>
	</div>
	<div class="column col-12">
		{#if saved}
			<div class="toast">Email Verification has been resent!</div>
		{/if}
		{#if errors}
			<div class="toast">{errors}</div>
		{/if}
		{#if loading}
			<div class="loading loading-lg"></div>
		{/if}
	</div>
</div>
