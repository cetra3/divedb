<script module lang="ts">
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
	import type { ClientError } from 'graphql-request';
	import { fbRegisterRedirect } from '$lib/util/fbRedirect';

	import type { PageData } from './$types';

	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();

	let fbAppId = data.fbAppId;

	const fbUrl = `https://www.facebook.com/v8.0/dialog/oauth?client_id=${fbAppId}&redirect_uri=${fbRegisterRedirect}&scope=email`;

	let username = $state('');
	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');

	let errors: string | undefined = $state(undefined);

	let loading = $state(false);

	let registered = $state(false);

	const onSubmit = (e: Event) => {
		e.preventDefault();

		loading = true;
		errors = undefined;
		client
			.registerUser({ username, email, password })
			.then((val) => {
				loading = false;
				registered = true;
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	let canSave = $derived(username != '' && email != '' && password != '' && password == confirmPassword);
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
			<form class="form-horizontal" onsubmit={onSubmit}>
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
					<button
						class="btn btn-primary"
						type="submit"
						disabled={canSave == false || registered == true}>Register</button
					>
				</FormRow>
			</form>
		</div>
		{#if registered}
			<div class="column col-12">
				<div class="toast">
					<p>
						Registration Successful! Please check your email account to continue with email
						verification.
					</p>
					<p>
						If the email doesn't come within a couple of minutes, please check your spam folder.
					</p>
				</div>
			</div>
		{/if}
		{#if errors}
			<div class="column col-12">
				<div class="toast">{errors}</div>
			</div>
		{/if}
		{#if loading}
			<div class="column col-12">
				<div class="loading loading-lg"></div>
			</div>
		{/if}
		{#if fbAppId != ''}
			<div class="column col-12 col-sm-12 padding-top">
				<a href={fbUrl} class="btn btn-primary">Register with Facebook</a>
			</div>
		{/if}
	</div>
</div>
