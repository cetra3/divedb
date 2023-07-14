<script lang="ts">
	import FormRow from '$lib/components/FormRow.svelte';
	import type { ClientError } from 'graphql-request';
	import type { CurrentUserFragment } from '$lib/graphql/generated';
	import { OverlayLocation } from '$lib/graphql/generated';
	import DeleteAccount from '$lib/components/forms/DeleteAccount.svelte';
	import ChangePassword from '$lib/components/forms/ChangePassword.svelte';
	import { onMount } from 'svelte';
	import { client } from '$lib/graphql/client';
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	let currentUser: CurrentUserFragment | undefined;
	let username = '';
	let displayName = '';
	let watermarkLocation = OverlayLocation.BottomRight;
	let copyrightLocation: OverlayLocation | '' = OverlayLocation.BottomLeft;

	onMount(async () => {
		currentUser = (await client.getCurrentUser()).currentUser ?? undefined;
		username = currentUser?.username ?? '';
		displayName = currentUser?.displayName ?? '';
		watermarkLocation = currentUser?.watermarkLocation ?? OverlayLocation.BottomLeft;
		copyrightLocation = currentUser?.copyrightLocation ?? ('' as OverlayLocation | '');
	});

	let pristine = true;

	let errors: string | undefined = undefined;

	let loading = false;

	let deleteModal = false;

	const onInput = () => {
		pristine = false;
	};

	const onSubmit = (e: Event) => {
		e.preventDefault();

		loading = true;

		errors = undefined;
		client
			.updateSettings({
				displayName,
				watermarkLocation,
				copyrightLocation: copyrightLocation == '' ? undefined : copyrightLocation
			})
			.then((val) => {
				loading = false;
				pristine = true;
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	$: canSave = displayName != '' && !pristine;
</script>

<svelte:head>
	<title>DiveDB - User Settings</title>
</svelte:head>

<CheckLogin />

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<h1 class="page-title">User Settings</h1>
		</div>
	</div>
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<p>Adjust your user settings below</p>
		</div>
	</div>
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<form class="form-horizontal" on:submit={onSubmit}>
				<FormRow name="Username">
					<input type="text" bind:value={username} disabled class="form-input" />
					<span class="form-input-hint"> This is the username you registered with </span>
				</FormRow>
				<FormRow name="Display Name">
					<input
						type="text"
						placeholder="Enter in a display name to use for watermarks and other things"
						bind:value={displayName}
						on:input={onInput}
						class="form-input"
					/>
					<span class="form-input-hint">
						This is the display name that will be displayed on watermarks and other places on DiveDB
					</span>
				</FormRow>
				<FormRow name="Logo Location">
					<select on:input={onInput} bind:value={watermarkLocation} class="form-select">
						<option value={OverlayLocation.BottomRight}> Bottom Right </option>
						<option value={OverlayLocation.BottomLeft}>Bottom Left</option>
						<option value={OverlayLocation.TopLeft}>Top Left</option>
						<option value={OverlayLocation.TopRight}>Top Right</option>
					</select>
					<span class="form-input-hint">
						This is where the DiveDB logo will be included on photos
					</span>
				</FormRow>
				<FormRow name="Copyright Location">
					<select on:input={onInput} bind:value={copyrightLocation} class="form-select">
						<option value={''}>Hidden</option>
						<option value={OverlayLocation.BottomRight}> Bottom Right </option>
						<option value={OverlayLocation.BottomLeft}>Bottom Left</option>
						<option value={OverlayLocation.TopLeft}>Top Left</option>
						<option value={OverlayLocation.TopRight}>Top Right</option>
					</select>
					<span class="form-input-hint">
						The copyright message is the year of the photo &amp; your username. I.e,{' '}
						<code>
							© {new Date().getFullYear()}
							{displayName}
						</code>
					</span>
				</FormRow>
				<FormRow name="">
					<button class="btn btn-primary" type="submit" disabled={canSave == false}
						>Save Changes</button
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
	</div>

	<ChangePassword />

	<div class="columns">
		<div class="column col-12 col-sm-12">
			<h3 class="padding-top">Account Deletion</h3>
		</div>
		<div class="column col-12 col-sm-12">
			<p>
				Delete your account and all content associated to you including photos, dive, logs, etc.
				Note: this is a non-reversible action, so make sure you have backups of your data
			</p>
		</div>
		<div class="column col-12 col-sm-12">
			<button
				class="btn btn-error"
				type="submit"
				on:click={() => {
					deleteModal = true;
				}}
			>
				Delete Account
			</button>
		</div>
	</div>

	{#if deleteModal}
		<DeleteAccount
			onClose={() => {
				deleteModal = false;
			}}
		/>
	{/if}
</div>
