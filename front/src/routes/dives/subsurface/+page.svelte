<script lang="ts">
	import FormRow from '$lib/components/FormRow.svelte';
	import { goto } from '$app/navigation';
	import type { ClientError } from 'graphql-request';
	import { session } from '$lib/session';
	import { client } from '$lib/graphql/client';

	let email = $session.user?.email ?? '';
	let password = '';

	let loading = false;

	let errors: string | undefined = undefined;

	$: canSave = email != '' && password != '';

	const onSubmit = (e: Event) => {
		e.preventDefault();
		errors = undefined;
		loading = true;
		client
			.syncSubsurface({ email, password })
			.then(() => {
				loading = false;
				goto('/dives');
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};
</script>

<svelte:head>
	<title>DiveDB - Sync from Subsurface</title>
</svelte:head>
<div class="container grid-lg">
	<div class="columns">
		<div class="column">
			<h1 class="page-title">Sync from Subsurface</h1>
			<p>Enter your Subsurface cloud account details to synchronise your dives and dive sites</p>
		</div>
	</div>

	<div class="columns">
		<div class="column col-12 col-sm-12">
			<form class="form-horizontal" on:submit={onSubmit}>
				<FormRow name="Subsurface Email">
					<input type="text" placeholder="Email" bind:value={email} class="form-input" />
				</FormRow>
				<FormRow name="Subsurface Password">
					<input type="password" placeholder="Password" bind:value={password} class="form-input" />
				</FormRow>
				<FormRow name="">
					<button class="btn btn-primary" type="submit" disabled={canSave == false}>Sync</button>
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
	</div>
</div>
