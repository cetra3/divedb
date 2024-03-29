<script context="module" lang="ts">
	import { client } from '$lib/graphql/client';

	export async function load() {
		let result = await client.fbAppId();

		return {
			props: {
				fbAppId: result.fbAppId
			}
		};
	}
</script>

<script lang="ts">
	import FormRow from '$lib/components/FormRow.svelte';
	import { goto } from '$app/navigation';
	import type { ClientError } from 'graphql-request';
	import { browser } from '$app/environment';
	import { session } from '$lib/session';
	import { fbRegisterRedirect } from '$lib/util/fbRedirect';

	import type { PageData } from './$types';

	export let data: PageData;

	let fbAppId = data.fbAppId;

	const fbUrl = `https://www.facebook.com/v8.0/dialog/oauth?client_id=${fbAppId}&redirect_uri=${fbRegisterRedirect}&scope=email`;

	let username = '';
	let email = '';
	let password = '';
	let confirmPassword = '';

	let query = new URLSearchParams(browser ? location.search : '');
	let redirect = query.get('redirect') || '/';

	let errors: string | undefined = undefined;

	let loading = false;

	const onSubmit = (e: Event) => {
		e.preventDefault();

		loading = true;
		errors = undefined;
		client
			.registerUser({ username, email, password })
			.then((val) => {
				localStorage.setItem('token', val.registerUser.token);
				session.set({ loggedIn: true, user: val.registerUser });
				loading = false;
				goto(redirect);
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	$: canSave = username != '' && email != '' && password != '' && password == confirmPassword;
</script>

<svelte:head>
	<title>DiveDB - Register</title>
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<h1 class="page-title">Register</h1>
		</div>
	</div>
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<p>Register below using your email &amp; password</p>
		</div>
	</div>
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<form class="form-horizontal" on:submit={onSubmit}>
				<FormRow name="Username">
					<input type="text" placeholder="Username" bind:value={username} class="form-input" />
				</FormRow>
				<FormRow name="Email">
					<input type="text" placeholder="Email" bind:value={email} class="form-input" />
				</FormRow>
				<FormRow name="Password">
					<input type="password" placeholder="Password" bind:value={password} class="form-input" />
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
					<button class="btn btn-primary" type="submit" disabled={canSave == false}>Register</button
					>
				</FormRow>
			</form>
		</div>
		{#if errors}
			<div class="toast">{errors}</div>
		{/if}
		{#if loading}
			<div class="column col-12">
				<div class="loading loading-lg" />
			</div>
		{/if}
		{#if fbAppId != ''}
			<div class="column col-12 col-sm-12 padding-top">
				<a href={fbUrl} class="btn btn-primary">Register with Facebook</a>
			</div>
		{/if}
	</div>
</div>
