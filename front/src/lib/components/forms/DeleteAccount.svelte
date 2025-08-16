<script lang="ts">
	import { goto } from '$app/navigation';
	import { session } from '$lib/session';
	import { client } from '$lib/graphql/client';
	import type { ClientError } from 'graphql-request';

	interface Props {
		onClose: () => void;
	}

	let { onClose }: Props = $props();
	let errors: string | undefined = $state(undefined);

	let password = $state('');
	let loading = $state(false);

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
	<div class="modal-overlay"></div>
	<div class="modal-container">
		<div class="modal-header">
			<!-- svelte-ignore a11y_missing_attribute -->
			<!-- svelte-ignore a11y_missing_content -->
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<a class="btn btn-clear float-right" aria-label="Close" onclick={onClose}></a>
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
						<div class="loading loading-lg"></div>
					</div>
				{/if}
			</div>
		</div>
		<div class="modal-footer">
			<button disabled={password == ''} onclick={onDelete} class="btn btn-error" type="submit">
				Delete Account
			</button>{' '}
			<button class="btn" onclick={onClose} type="submit"> Cancel </button>
		</div>
	</div>
</div>
