<script lang="ts">
	import { goto } from '$app/navigation';
	import SealifeIcon from '$lib/icons/SealifeIcon.svelte';
	import type { CreateSealife } from '$lib/graphql/generated';
	import type { PageData } from './$types';
	import EditSealife from '$lib/components/forms/EditSealife.svelte';
	import { session } from '$lib/session';
	import { client } from '$lib/graphql/client';
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	import References from '$lib/components/References.svelte';

	export let data: PageData;
	$: sealife = data.sealife;
	$: categories = data.categories;

	let showRemove = false;

	$: isEditor = $session.user?.level == 'ADMIN' || $session.user?.level == 'EDITOR';

	let onRemove = () => {
		if (sealife) {
			client.removeSealife({ id: sealife.id }).then((_) => {
				goto(`/sealife`);
			});
		}
	};

	const onShow = () => {
		showRemove = true;
	};

	const onClose = () => {
		showRemove = false;
	};

	let onSave = (sealife: CreateSealife) => {
		client.addSealife({ sealife }).then((val) => {
			goto(`/sealife/${val.newSealife.slug}`);
		});
	};
</script>

<CheckLogin />

<div class="container grid-lg">
	<div class="columns">
		<div class="column">
			<h1 class="page-title">
				<SealifeIcon size="33px" /> Edit Sealife
				{#if isEditor}
					<button class="btn btn-secondary btn-sm" on:click={onShow}> Remove </button>
				{/if}
			</h1>
		</div>
	</div>

	{#if sealife}
		<EditSealife {categories} {onSave} {sealife} />
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
						<div class="modal-title h5">Remove Site</div>
					</div>
					<div class="modal-body">
						<div class="content">
							Are you sure you want to remove {sealife.name}?
						</div>
					</div>
					<div class="modal-footer">
						<button class="btn btn-primary" on:click={onRemove}> Remove Sealife </button>{' '}
						<button on:click={onClose} class="btn btn-secondary"> Cancel </button>
					</div>
				</div>
			</div>
		{/if}
	{/if}
</div>
