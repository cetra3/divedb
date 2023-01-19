<script lang="ts">
	import { goto } from '$app/navigation';
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	import EditRegion from '$lib/components/forms/EditRegion.svelte';

	import { client } from '$lib/graphql/client';
	import type { CreateRegion } from '$lib/graphql/generated';
	import type { ClientError } from 'graphql-request';

	import type { PageData } from './$types';
	export let data: PageData;
	let region = data.region;

	let errors: string | undefined = undefined;

	let showRemove = false;

	let onRemove = () => {
		if (region) {
			client.removeRegion({ id: region.id }).then(() => {
				goto(`/admin`);
			});
		}
	};

	const onShow = () => {
		showRemove = true;
	};

	const onClose = () => {
		showRemove = false;
	};

	const onSave = (region: CreateRegion) => {
		loading = true;
		client
			.newRegion({ region })
			.then(() => {
				loading = false;
				goto('/admin');
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	let loading = false;
</script>

<CheckLogin />
<svelte:head>
	<title>DiveDB - Edit Region</title>
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12">
			<h1 class="page-title">
				Edit Region
				<button class="btn btn-secondary btn-sm" on:click={onShow}> Remove </button>
			</h1>
		</div>
		<div class="column col-12">
			<EditRegion {region} {onSave} />

			{#if showRemove}
				<div class={`modal active`}>
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<span class="modal-overlay" aria-label="Close" on:click={onClose} />
					<div class="modal-container">
						<div class="modal-header">
							<!-- svelte-ignore a11y-missing-content -->
							<!-- svelte-ignore a11y-invalid-attribute -->
							<a
								href="javascript:void(0)"
								class="btn btn-clear float-right"
								aria-label="Close"
								on:click={onClose}
							/>
							<div class="modal-title h5">Remove Region</div>
						</div>
						<div class="modal-body">
							<div class="content">Are you sure you want to remove this region?</div>
						</div>
						<div class="modal-footer">
							<button class="btn btn-primary" on:click={onRemove}> Remove Region </button>{' '}
							<button on:click={onClose} class="btn btn-secondary"> Cancel </button>
						</div>
					</div>
				</div>
			{/if}
		</div>
		{#if errors}
			<div class="column col-12">
				<div class="toast">{errors}</div>
			</div>
		{/if}
		{#if loading}
			<div class="column col-12">
				<div class="loading loading-lg" />
			</div>
		{/if}
	</div>
</div>
