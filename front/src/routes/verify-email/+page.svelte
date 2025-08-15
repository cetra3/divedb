<script lang="ts">
	import FormRow from '$lib/components/FormRow.svelte';
	import { client } from '$lib/graphql/client';
	import type { ClientError } from 'graphql-request';
	import { browser } from '$app/environment';
	import { session } from '$lib/session';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	let query = new URLSearchParams(browser ? location.search : '');

	let email = query.get('email');
	let token = query.get('token');

	let errors: string | undefined = undefined;

	let loading = false;

	onMount(() => {
		if (!email || !token) {
			errors = 'Invalid Email Verification Link.';
		} else {
			loading = true;
			errors = undefined;
			client
				.verifyEmail({ email, token })
				.then((val) => {
					loading = false;
					localStorage.setItem('token', val.verifyEmail.token);
					session.set({ loggedIn: true, user: val.verifyEmail });
					goto('/');
				})
				.catch((reason: ClientError) => {
					loading = false;
					errors = reason.response.errors?.map((val) => val.message).join();
				});
		}
	});
</script>

<svelte:head>
	<title>DiveDB - Email Verification</title>
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<h1 class="page-title">Email Verification</h1>
		</div>
	</div>
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<p>Set a new password below to finish resetting your password</p>
		</div>
	</div>
	<div class="columns">
		{#if errors}
			<div class="toast">{errors}</div>
		{/if}
		{#if loading}
			<div class="column col-12">
				<div class="loading loading-lg"></div>
			</div>
		{/if}
	</div>
</div>
