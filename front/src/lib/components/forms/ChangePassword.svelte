<script lang="ts">
	import { client } from '$lib/graphql/client';
	import type { ClientError } from 'graphql-request';

	import FormRow from '../FormRow.svelte';

	let oldPassword = '';
	let newPassword = '';
	let confirmPassword = '';

	let errors: string | undefined = undefined;

	let loading = false;
	let pristine = true;
	let saved = false;

	const onInput = () => {
		saved = false;
		pristine = false;
	};

	const onSubmit = (e: Event) => {
		e.preventDefault();

		loading = true;
		errors = undefined;
		client
			.changePassword({ oldPassword, newPassword })
			.then(() => {
				loading = false;
				pristine = true;
				oldPassword = '';
				newPassword = '';
				confirmPassword = '';
				saved = true;
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	$: canSave =
		oldPassword != '' && newPassword != '' && newPassword == confirmPassword && !pristine;
</script>

<div class="columns">
	<div class="column col-12 col-sm-12"><h3 class="page-title padding-top">Change Password</h3></div>
	<div class="column col-12 col-sm-12"><p>Change your password below</p></div>
</div>
<div class="columns">
	<div class="column col-12 col-sm-12">
		<form class="form-horizontal" on:submit={onSubmit}>
			<FormRow name="Current Password">
				<input
					on:input={onInput}
					type="password"
					placeholder="Enter your current password here"
					bind:value={oldPassword}
					class="form-input"
				/>
			</FormRow>
			<FormRow name="New Password">
				<input
					on:input={onInput}
					type="password"
					placeholder="Enter your new password here"
					bind:value={newPassword}
					class="form-input"
				/>
			</FormRow>
			<FormRow name="Confirm Password">
				<input
					on:input={onInput}
					type="password"
					placeholder="Confirm your new password here"
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
	{#if saved}
		<div class="toast">Password has been changed!</div>
	{/if}
	{#if errors}
		<div class="toast">{errors}</div>
	{/if}
	{#if loading}
		<div class="column col-12">
			<div class="loading loading-lg" />
		</div>
	{/if}
</div>
