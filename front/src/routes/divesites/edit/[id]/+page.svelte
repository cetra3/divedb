<script lang="ts">
	import { goto } from '$app/navigation';
	import EditDiveSite from '$lib/components/forms/EditDiveSite.svelte';
	import type { CreateDiveSite, SiteFragment } from '$lib/graphql/generated';
	import DiveSiteIcon from '$lib/icons/DiveSiteIcon.svelte';
	import type { PageData } from './$types';
	let diveSite = data.diveSite;
	import { session } from '$lib/session';
	import { client } from '$lib/graphql/client';
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();

	let showRemove = $state(false);

	let isEditor =
		$derived($session.user?.level == 'ADMIN' ||
		$session.user?.level == 'EDITOR' ||
		($session.user?.id != undefined && $session.user.id === diveSite?.userId));

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
					<button class="btn btn-secondary btn-sm" onclick={onShow}> Remove </button>
				{/if}
			</h1>
		</div>
	</div>

	{#if diveSite}
		<EditDiveSite {onSave} site={diveSite} />
		{#if showRemove}
			<div class={`modal active`}>
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<span class="modal-overlay" aria-label="Close" onclick={onClose}></span>
				<div class="modal-container">
					<div class="modal-header">
						<!-- svelte-ignore a11y_missing_content -->
						<!-- svelte-ignore a11y_invalid_attribute -->
						<a
							href="javascript:void(0)"
							class="btn btn-clear float-right"
							aria-label="Close"
							onclick={onClose}
						></a>
						<div class="modal-title h5">Remove Site</div>
					</div>
					<div class="modal-body">
						<div class="content">
							Are you sure you want to remove {diveSite.name}?
						</div>
					</div>
					<div class="modal-footer">
						<button class="btn btn-primary" onclick={onRemove}> Remove Site </button>{' '}
						<button onclick={onClose} class="btn btn-secondary"> Cancel </button>
					</div>
				</div>
			</div>
		{/if}
	{/if}
</div>
