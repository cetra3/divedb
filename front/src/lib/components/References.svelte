<script lang="ts">
	import { client } from '$lib/graphql/client';
	import type { ReferenceFragment } from '$lib/graphql/generated';
	import type { ClientError } from 'graphql-request';
	import FormRow from './FormRow.svelte';

	export let references: ReferenceFragment[];
	export let sealifeId: string | undefined = undefined;
	export let diveSiteId: string | undefined = undefined;
	export let showEdit = true;

	let showForm = false;
	let url = '';

	$: canSave = url != '';

	let errors: string | undefined = undefined;
	let loading = false;

	let toRemove: string | undefined = undefined;

	const onSubmit = (e: Event) => {
		e.preventDefault();

		loading = true;
		errors = undefined;

		client
			.newReference({ url, sealifeId, diveSiteId })
			.then((val) => {
				loading = false;
				references = [val.newReference, ...references];
				showForm = false;
				url = '';
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	const onRemove = (id: string) => {
		loading = true;
		errors = undefined;

		client
			.removeReference({ id })
			.then(() => {
				loading = false;
				references = references.filter((val) => val.id !== id);
				toRemove = undefined;
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	const urlDisplay = (val: string) => {
		return new URL(val).host;
	};
</script>

{#if showEdit || references.length > 0}
	<div class="divider" />

	<h5>
		References

		{#if showEdit}
			<button
				class="btn btn-sm"
				on:click={() => {
					showForm = !showForm;
				}}
			>
				Edit</button
			>
		{/if}
	</h5>
{/if}
{#if showForm}
	<form class="form-horizontal" on:submit={onSubmit}>
		<FormRow name="URL">
			<input type="text" placeholder="https://divedb.net" bind:value={url} class="form-input" />
		</FormRow>

		<FormRow name="">
			<button class="btn btn-primary" type="submit" disabled={canSave == false}
				>Add Reference</button
			>
		</FormRow>
	</form>
{/if}
{#if errors}
	<div class="toast">{errors}</div>
{/if}
{#if loading}
	<div class="loading loading-lg" />
{/if}

<ul>
	{#each references as reference}
		<li>
			<a href={reference.url}>{reference.title}</a>
			<em class="text-muted text-small"> - {urlDisplay(reference.url)}</em>
			{#if showForm}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<button
					class="btn btn-sm"
					on:click={() => {
						if (toRemove !== reference.id) {
							toRemove = reference.id;
						} else {
							toRemove = undefined;
						}
					}}>Remove</button
				>
			{/if}
			{#if toRemove == reference.id}
				<br /> Are you sure you want to remove this reference?
				<button
					class="btn btn-sm"
					on:click={() => {
						if (toRemove !== undefined) {
							onRemove(toRemove);
						}
					}}>Yes</button
				>
				/
				<button
					class="btn btn-sm"
					on:click={() => {
						toRemove = undefined;
					}}>No</button
				>
			{/if}
		</li>
	{/each}
</ul>
