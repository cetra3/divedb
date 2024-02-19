<script lang="ts">
	import FormRow from '$lib/components/FormRow.svelte';
	import type { ClientError } from 'graphql-request';
	import type { CurrentUserFragment, PhotoSummaryFragment } from '$lib/graphql/generated';
	import { OverlayLocation } from '$lib/graphql/generated';
	import DeleteAccount from '$lib/components/forms/DeleteAccount.svelte';
	import ChangePassword from '$lib/components/forms/ChangePassword.svelte';
	import { onMount } from 'svelte';
	import { client } from '$lib/graphql/client';
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	import ImageUpload from '$lib/components/forms/ImageUpload.svelte';
	let currentUser: CurrentUserFragment | undefined;
	let username = '';
	let displayName = '';
	let watermarkLocation = OverlayLocation.BottomRight;
	let copyrightLocation: OverlayLocation | '' = OverlayLocation.BottomLeft;
	let description = '';
	let photoId: string | undefined = undefined;

	onMount(async () => {
		currentUser = (await client.getCurrentUser()).currentUser ?? undefined;
		username = currentUser?.username ?? '';
		displayName = currentUser?.displayName ?? '';
		watermarkLocation = currentUser?.watermarkLocation ?? OverlayLocation.BottomLeft;
		copyrightLocation = currentUser?.copyrightLocation ?? ('' as OverlayLocation | '');
		description = currentUser?.description ?? '';
		photoId = currentUser?.photoId ?? undefined;
	});

	let pristine = true;

	let errors: string | undefined = undefined;

	let loading = false;

	let deleteModal = false;

	let file: File | undefined = undefined;

	const onUpload = (e: Event) => {
		let files = (e.target as any).files;

		if (files && files.length > 0) {
			file = files.item(0);
		}
	};

	const removePhoto = () => {
		photoId = undefined;
		pristine = false;
	};

	const onUploaded = (e: CustomEvent<{ photo: PhotoSummaryFragment; index: number }>) => {
		const { photo } = e.detail;

		file = undefined;

		photoId = photo.id;
		pristine = false;
	};

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
				copyrightLocation: copyrightLocation == '' ? undefined : copyrightLocation,
				photoId,
				description
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
				<FormRow name="Profile Photo">
					<div class="columns">
						{#if photoId}
							<div class="column col-3 col-sm-6">
								<!-- svelte-ignore a11y-missing-attribute -->
								<img
									src={`/api/photos/jpeg/${photoId}`}
									class="img-edit img-responsive"
									style="margin-bottom: 0.4rem;"
								/>
							</div>
						{/if}
						{#if file}
							<ImageUpload index={0} internal={true} on:upload={onUploaded} {file} />
						{/if}
					</div>
					<div class="columns">
						{#if photoId}
							<div class="column col-3 col-sm-6">
								<button
									class="btn btn-sm btn-secondary btn-block"
									on:click={removePhoto}
									type="button">Remove Photo</button
								>
							</div>
						{/if}
						<div class="column col-3 col-sm-6">
							<input on:change={onUpload} id="fileupload" type="file" accept=".jpg,.jpeg" />
						</div>
					</div>
				</FormRow>
				<FormRow name="Description">
					<textarea
						placeholder="Enter in a description of yourself, your dive certifications, and what you like to do!"
						bind:value={description}
						on:input={onInput}
						rows="8"
						class="form-input"
					/>
					<span class="form-input-hint">
						This description will be displayed on your public profile
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
					<a class="btn btn-secondary" href="/users/{username}">View Profile</a>
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
