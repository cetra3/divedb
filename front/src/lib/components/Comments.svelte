<script lang="ts">
	import { client } from '$lib/graphql/client';
	import type { DiveNodeFragment } from '$lib/graphql/generated';
	import type { ClientError } from 'graphql-request';
	import UserLabel from './labels/UserLabel.svelte';

	import { session } from '$lib/session';
	import { goto } from '$app/navigation';
	interface Props {
		dive: DiveNodeFragment;
		showEdit?: boolean;
	}

	let { dive = $bindable(), showEdit = true }: Props = $props();

	let showForm = $state(false);
	let description = $state('');

	let comments = $derived(dive.comments);
	let canSave = $derived(description != '');

	let errors: string | undefined = $state(undefined);
	let loading = $state(false);

	let toRemove: string | undefined = $state(undefined);

	const onAddNew = () => {
		if (!$session.loggedIn) {
			let loginPath = `/login?redirect=${location.pathname}`;
			goto(loginPath);
		} else {
			showForm = !showForm;
		}
	};

	const onSubmit = (e: Event) => {
		e.preventDefault();

		loading = true;
		errors = undefined;

		client
			.addComment({ diveId: dive.id, description })
			.then((val) => {
				loading = false;
				dive.comments = [val.newComment, ...comments];
				dive.numComments = dive.comments.length;
				showForm = false;
				description = '';
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	const onRemove = (commentId: string) => {
		loading = true;
		errors = undefined;

		client
			.removeComment({ commentId })
			.then(() => {
				loading = false;
				dive.comments = comments.filter((val) => val.id !== commentId);
				dive.numComments = dive.comments.length;
				toRemove = undefined;
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};
</script>

<h5 id="comments">
	Comments

	{#if showEdit}
		<button class="btn btn-sm" onclick={onAddNew}> Add New</button>
	{/if}
</h5>
{#if showForm}
	<form class="form-group" onsubmit={onSubmit}>
		<div class="form-group">
			<textarea rows={4} bind:value={description} class="form-input"></textarea>
		</div>
		<div class="form-group">
			<button class="btn btn-primary" type="submit" disabled={canSave == false}>Add Comment</button>
		</div>
	</form>
{/if}
{#if errors}
	<div class="toast">{errors}</div>
{/if}
{#if loading}
	<div class="loading loading-lg"></div>
{/if}

{#each comments as comment}
	<div class="card">
		<div class="card-body">
			{comment.description}
		</div>
		<div class="card-footer">
			<UserLabel user={comment.user} />
			<span class="label">{new Date(comment.date).toLocaleString()}</span>
			{#if $session.user?.id != undefined && $session.user.id === comment.user.id}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<span
					onclick={() => {
						if (toRemove !== comment.id) {
							toRemove = comment.id;
						} else {
							toRemove = undefined;
						}
					}}
					class="label label-secondary pointer"
				>
					Remove
				</span>
			{/if}
			{#if toRemove == comment.id}
				<br /> Are you sure you want to remove this comment?
				<button
					class="btn btn-sm"
					onclick={() => {
						if (toRemove !== undefined) {
							onRemove(toRemove);
						}
					}}>Yes</button
				>
				/
				<button
					class="btn btn-sm"
					onclick={() => {
						toRemove = undefined;
					}}>No</button
				>
			{/if}
		</div>
	</div>
{/each}
