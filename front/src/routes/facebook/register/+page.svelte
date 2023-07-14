<script lang="ts">
	import { onMount } from 'svelte';
	import { client } from '$lib/graphql/client';
	import { goto } from '$app/navigation';
	import { session } from '$lib/session';
	import { fbRegisterRedirect } from '$lib/util/fbRedirect';
	import type { ClientError } from 'graphql-request';
	import FormRow from '$lib/components/FormRow.svelte';

	let errors: string | undefined = undefined;
	let username = '';

	let loading = false;

	const onSubmit = (e: Event) => {
		e.preventDefault();
		let searchParams = new URLSearchParams(window.location.search);

		errors = undefined;
		loading = true;
		let code = searchParams.get('code') ?? '';
		let redirect = searchParams.get('redirect') || '/';

		client
			.fbRegisterUser({ username, redirectUri: fbRegisterRedirect, code })
			.then((val) => {
				localStorage.setItem('token', val.fbRegisterUser.token);
				session.set({ loggedIn: true, user: val.fbRegisterUser });
				loading = false;
				goto(redirect);
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	$: canSave = username != '';
</script>

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<h1 class="page-title">Complete Facebook Registration</h1>
		</div>
		{#if errors}
			<div class="toast">{errors}</div>
		{/if}
		{#if loading}
			<div class="column col-12">
				<div class="loading loading-lg" />
			</div>
		{/if}
		<div class="column col-12 col-sm-12">
			<form class="form-horizontal" on:submit={onSubmit}>
				<FormRow name="Username">
					<input type="text" bind:value={username} class="form-input" />
					<span class="form-input-hint"> Enter in a username to use with this account </span>
				</FormRow>
				<FormRow name="">
					<button class="btn btn-primary" type="submit" disabled={canSave == false}
						>Save Changes</button
					>
				</FormRow>
			</form>
		</div>
	</div>
</div>
