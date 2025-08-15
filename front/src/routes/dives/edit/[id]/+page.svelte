<script lang="ts">
	import { goto } from '$app/navigation';
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	import EditDive from '$lib/components/forms/EditDive.svelte';
	import { client } from '$lib/graphql/client';
	import type { CreateDive } from '$lib/graphql/generated';
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';

	import type { PageData } from './$types';
	export let data: PageData;
	let dive = data.dive;
	let showRemove = false;

	let onRemove = () => {
		if (dive) {
			client.removeDive({ id: dive.id }).then(() => {
				goto(`/dives`);
			});
		}
	};

	const onShow = () => {
		showRemove = true;
	};

	const onClose = () => {
		showRemove = false;
	};

	let onSave = (dive: CreateDive) => {
		client.addDive({ dive }).then((val) => {
			goto(`/dives/${val.newDive.id}`);
		});
	};
</script>

<svelte:head>
	<title>DiveDB - Edit Dive</title>
</svelte:head>

<CheckLogin />

<div class="container grid-lg">
	<div class="columns">
		<div class="column">
			<h1 class="page-title">
				<DiveLogIcon size="33px" /> Edit Dive
				<button class="btn btn-secondary btn-sm" on:click={onShow}> Remove </button>
			</h1>
		</div>
	</div>

	{#if dive != undefined}
		<EditDive {onSave} {dive} />
		{#if showRemove}
			<div class={`modal active`}>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<span class="modal-overlay" aria-label="Close" on:click={onClose}></span>
				<div class="modal-container">
					<div class="modal-header">
						<!-- svelte-ignore a11y-missing-content -->
						<!-- svelte-ignore a11y-invalid-attribute -->
						<a
							href="javascript:void(0)"
							class="btn btn-clear float-right"
							aria-label="Close"
							on:click={onClose}
						></a>
						<div class="modal-title h5">Remove Site</div>
					</div>
					<div class="modal-body">
						<div class="content">Are you sure you want to remove this Dive?</div>
					</div>
					<div class="modal-footer">
						<button class="btn btn-primary" on:click={onRemove}> Remove Dive </button>{' '}
						<button on:click={onClose} class="btn btn-secondary"> Cancel </button>
					</div>
				</div>
			</div>
		{/if}
	{:else}
		<div class="column col-12">
			<div class="loading loading-lg"></div>
		</div>
	{/if}
</div>
