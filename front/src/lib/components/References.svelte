<script lang="ts">
	import { client } from '$lib/graphql/client';
	import type { ReferenceFragment } from '$lib/graphql/generated';
	import type { ClientError } from 'graphql-request';
	import FormRow from './FormRow.svelte';

	interface Props {
		references: ReferenceFragment[];
		sealifeId?: string | undefined;
		diveSiteId?: string | undefined;
		showEdit?: boolean;
	}

	let {
		references = $bindable(),
		sealifeId = undefined,
		diveSiteId = undefined,
		showEdit = true
	}: Props = $props();

	let showForm = $state(false);
	let url = $state('');

	let canSave = $derived(url != '');

	let errors: string | undefined = $state(undefined);
	let loading = $state(false);

	let toRemove: string | undefined = $state(undefined);

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
	<div class="divider"></div>

	<h5>
		References

		{#if showEdit}
			<button
				class="btn btn-sm"
				onclick={() => {
					showForm = !showForm;
				}}
			>
				Edit</button
			>
		{/if}
	</h5>
{/if}
{#if showForm}
	<form class="form-horizontal" onsubmit={onSubmit}>
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
	<div class="loading loading-lg"></div>
{/if}

<ul>
	{#each references as reference}
		<li>
			<a href={reference.url}>{reference.title}</a>
			<em class="text-muted text-small"> - {urlDisplay(reference.url)}</em>
			{#if showForm}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<button
					class="btn btn-sm"
					onclick={() => {
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
		</li>
	{/each}
</ul>
