<script lang="ts">
	import { goto } from '$app/navigation';
	import { session } from '$lib/session';
	import { client } from '$lib/graphql/client';
	import type { ClientError } from 'graphql-request';

	export let onClose: () => void;
	let errors: string | undefined = undefined;

	let password = '';
	let loading = false;

	const onDelete = () => {
		client
			.deleteUser({ password })
			.then(() => {
				localStorage.removeItem('token');
				session.set({ loggedIn: false });
				goto('/');
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};
</script>

<div class="modal active">
	<div class="modal-overlay" />
	<div class="modal-container">
		<div class="modal-header">
			<!-- svelte-ignore a11y-missing-attribute -->
			<!-- svelte-ignore a11y-missing-content -->
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<a class="btn btn-clear float-right" aria-label="Close" on:click={onClose} />
			<div class="modal-title h5">Delete Account</div>
		</div>
		<div class="modal-body">
			<div class="content">
				<p>
					Deleting your account is <strong>permanent</strong>. Any content associated to you will be
					deleted and we won't be able to recover any lost data.
				</p>
				<p>Enter your account password below to continue</p>
				<p>
					<input class="form-input" type="password" placeholder="Password" bind:value={password} />
				</p>

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
		<div class="modal-footer">
			<button disabled={password == ''} on:click={onDelete} class="btn btn-error" type="submit">
				Delete Account
			</button>{' '}
			<button class="btn" on:click={onClose} type="submit"> Cancel </button>
		</div>
	</div>
</div>
