<script lang="ts">
	import { goto } from '$app/navigation';
	import EditDiveSite from '$lib/components/forms/EditDiveSite.svelte';
	import type { CreateDiveSite, SiteFragment } from '$lib/graphql/generated';
	import DiveSiteIcon from '$lib/icons/DiveSiteIcon.svelte';
	import type { PageData } from './$types';
	export let data: PageData;
	let diveSite = data.diveSite;
	import { session } from '$lib/session';
	import { client } from '$lib/graphql/client';
	import CheckLogin from '$lib/components/CheckLogin.svelte';

	let showRemove = false;

	$: isEditor =
		$session.user?.level == 'ADMIN' ||
		$session.user?.level == 'EDITOR' ||
		($session.user?.id != undefined && $session.user.id === diveSite?.userId);

	let onRemove = () => {
		if (diveSite) {
			client.removeDiveSite({ id: diveSite.id }).then(() => {
				goto(`/divesites`);
			});
		}
	};

	const onShow = () => {
		showRemove = true;
	};

	const onClose = () => {
		showRemove = false;
	};

	let onSave = (site: CreateDiveSite) => {
		client.addDiveSite({ site }).then((val) => {
			goto(`/sites/${val.newDiveSite.slug}`);
		});
	};
</script>

<CheckLogin />
<div class="container grid-lg">
	<div class="columns">
		<div class="column">
			<h1 class="page-title">
				<DiveSiteIcon size="33px" /> Edit Dive Site
				{#if isEditor}
					<button class="btn btn-secondary btn-sm" on:click={onShow}> Remove </button>
				{/if}
			</h1>
		</div>
	</div>

	{#if diveSite}
		<EditDiveSite {onSave} site={diveSite} />
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
							Are you sure you want to remove {diveSite.name}?
						</div>
					</div>
					<div class="modal-footer">
						<button class="btn btn-primary" on:click={onRemove}> Remove Site </button>{' '}
						<button on:click={onClose} class="btn btn-secondary"> Cancel </button>
					</div>
				</div>
			</div>
		{/if}
	{/if}
</div>
