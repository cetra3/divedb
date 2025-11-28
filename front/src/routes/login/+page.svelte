<script lang="ts">
	import { goto } from '$app/navigation';
	import type { ClientError } from 'graphql-request';
	import { browser } from '$app/environment';
	import { fbLoginRedirect } from '$lib/util/fbRedirect';
	import FormRow from '$lib/components/FormRow.svelte';
	import { session } from '$lib/session';
	import { client } from '$lib/graphql/client';
	import type { PageData } from './$types';

	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();

	let fbAppId = data.loginInfo.fbAppId;
	let openidIssuerName = data.loginInfo.openidIssuerName;
	let disableEmailLogin = data.loginInfo.disableEmailLogin;

	let email = $state('');
	let password = $state('');

	let query = new URLSearchParams(browser ? location.search : '');
	let redirect = query.get('redirect') || '/';

	let errors: string | undefined = $state(undefined);

	let loading = $state(false);

	const fbUrl = `https://www.facebook.com/v8.0/dialog/oauth?client_id=${fbAppId}&redirect_uri=${fbLoginRedirect}&scope=email`;

	const onOauthLogin = (e: Event) => {
		e.preventDefault();
		client
			.oauthAuthorizationUrl()
			.then((response) => {
				window.location.href = response.oauthAuthorizationUrl;
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	const onSubmit = (e: Event) => {
		e.preventDefault();

		loading = true;
		errors = undefined;
		client
			.loginUser({ email, password })
			.then((val) => {
				localStorage.setItem('token', val.login.token);
				session.set({ loggedIn: true, user: val.login });
				loading = false;
				goto(redirect);
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	let canSave = $derived(email != '' && password != '');
</script>

<svelte:head>
	<title>DiveDB - Login</title>
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<h1 class="page-title">Login</h1>
		</div>
	</div>
	{#if !disableEmailLogin}
		<div class="columns">
			<div class="column col-12 col-sm-12">
				<p>Login below using your email &amp; password</p>
			</div>
		</div>
	{/if}
	<div class="columns">
		{#if !disableEmailLogin}
			<div class="column col-12 col-sm-12">
				<form class="form-horizontal" onsubmit={onSubmit}>
					<FormRow name="Email">
						<input type="text" placeholder="Email" bind:value={email} class="form-input" />
					</FormRow>
					<FormRow name="Password">
						<input
							type="password"
							placeholder="Password"
							bind:value={password}
							class="form-input"
						/>
					</FormRow>
					<FormRow name="">
						<button class="btn btn-primary" type="submit" disabled={canSave == false}>Login</button>
					</FormRow>
				</form>
			</div>
		{/if}
		{#if errors}
			<div class="toast">{errors}</div>
		{/if}
		{#if loading}
			<div class="column col-12">
				<div class="loading loading-lg"></div>
			</div>
		{/if}

		<div class="column col-12 col-sm-12 padding-top">
			<p>Don't have an account? <a href="/register?redirect={redirect}">Register here</a>.</p>
			{#if !disableEmailLogin}
				<p>
					Forgot your password? <a href="/forgot-password?redirect={redirect}"
						>Reset Your Password</a
					>.
				</p>
			{/if}
		</div>

		{#if fbAppId != ''}
			<div class="column col-12 col-sm-12 padding-top">
				<a href={fbUrl} class="btn btn-primary">Login with Facebook</a>
			</div>
		{/if}

		{#if openidIssuerName != ''}
			<div class="column col-12 col-sm-12 padding-top">
				<button onclick={onOauthLogin} class="btn btn-primary">Login with {openidIssuerName}</button
				>
			</div>
		{/if}
	</div>
</div>
