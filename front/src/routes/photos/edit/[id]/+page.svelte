<script lang="ts">
	import { goto } from '$app/navigation';
	import type { CreatePhoto } from '$lib/graphql/generated';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import EditPhoto from '$lib/components/forms/EditPhoto.svelte';

	import type { PageData } from './$types';
	export let data: PageData;

	$: photo = data.photo;

	import Photo from '$lib/components/photos/Photo.svelte';
	import { session } from '$lib/session';
	import { client } from '$lib/graphql/client';
	import CheckLogin from '$lib/components/CheckLogin.svelte';

	let showRemove = false;

	$: isEditor = $session.user?.level == 'ADMIN' || $session.user?.level == 'EDITOR';

	let onRemove = () => {
		if (photo) {
			client.removePhoto({ id: photo.id ?? '' }).then(() => {
				goto(`/photos`);
			});
		}
	};

	const onShow = () => {
		showRemove = true;
	};

	const onClose = () => {
		showRemove = false;
	};

	let onSave = (photo: CreatePhoto) => {
		client.updatePhoto({ photo }).then(() => {
			goto(`/photos/${photo.id}`);
		});
	};
</script>

<CheckLogin />

<div class="container grid-lg">
	<div class="columns">
		<div class="column">
			<h1 class="page-title">
				<PhotoIcon size="33px" /> Edit photo
				{#if isEditor}
					<button class="btn btn-secondary btn-sm" on:click={onShow}> Remove </button>
				{/if}
			</h1>
		</div>
	</div>

	{#if photo}
		{#if photo?.id !== undefined}
			<div class="column col-12">
				<Photo alt={'Photo to edit'} imgClass="img-responsive img-edit" id={photo.id ?? ''} />
			</div>
		{/if}
		<div class="column col-12">
			<EditPhoto {onSave} {photo} />
		</div>
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
						<div class="content">Are you sure you want to remove this photo?</div>
					</div>
					<div class="modal-footer">
						<button class="btn btn-primary" on:click={onRemove}> Remove photo </button>{' '}
						<button on:click={onClose} class="btn btn-secondary"> Cancel </button>
					</div>
				</div>
			</div>
		{/if}
	{/if}
</div>
