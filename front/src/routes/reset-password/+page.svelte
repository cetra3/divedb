<script lang="ts">
	import FormRow from '$lib/components/FormRow.svelte';
	import { client } from '$lib/graphql/client';
	import type { ClientError } from 'graphql-request';
	import { browser } from '$app/environment';
	import { session } from '$lib/session';
	import { goto } from '$app/navigation';

	let query = new URLSearchParams(browser ? location.search : '');

	let email = query.get('email') || '';
	let token = query.get('token') || '';
	let newPassword = '';
	let confirmPassword = '';

	let errors: string | undefined = undefined;

	let loading = false;

	const onSubmit = (e: Event) => {
		e.preventDefault();

		loading = true;
		errors = undefined;
		client
			.resetPassword({ email, newPassword, token })
			.then((val) => {
				loading = false;
				localStorage.setItem('token', val.resetPassword.token);
				session.set({ loggedIn: true, user: val.resetPassword });
				goto('/');
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	$: canSave = email != '' && newPassword != '' && newPassword == confirmPassword;
</script>

<svelte:head>
	<title>DiveDB - Reset Password</title>
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<h1 class="page-title">Password Reset</h1>
		</div>
	</div>
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<p>Set a new password below to finish resetting your password</p>
		</div>
	</div>
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<form class="form-horizontal" on:submit={onSubmit}>
				<FormRow name="Email">
					<input type="text" placeholder="Email" bind:value={email} class="form-input" />
				</FormRow>
				<FormRow name="New Password">
					<input
						type="password"
						placeholder="Password"
						bind:value={newPassword}
						class="form-input"
					/>
				</FormRow>
				<FormRow name="Confirm Password">
					<input
						type="password"
						placeholder="Confirm Password"
						bind:value={confirmPassword}
						class="form-input"
					/>
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
	</div>
</div>
