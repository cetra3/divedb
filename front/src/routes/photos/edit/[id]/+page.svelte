<script lang="ts">
	import { goto } from '$app/navigation';
	import type { CreatePhoto } from '$lib/graphql/generated';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import EditPhoto from '$lib/components/forms/EditPhoto.svelte';

	import type { PageData } from './$types';


	import Photo from '$lib/components/photos/Photo.svelte';
	import { session } from '$lib/session';
	import { client } from '$lib/graphql/client';
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();

	let showRemove = $state(false);


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
	let photo = $derived(data.photo);
	let isEditor = $derived($session.user?.level == 'ADMIN' || $session.user?.level == 'EDITOR');
</script>

<CheckLogin />

<div class="container grid-lg">
	<div class="columns">
		<div class="column">
			<h1 class="page-title">
				<PhotoIcon size="33px" /> Edit photo
				{#if isEditor}
					<button class="btn btn-secondary btn-sm" onclick={onShow}> Remove </button>
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
						<div class="content">Are you sure you want to remove this photo?</div>
					</div>
					<div class="modal-footer">
						<button class="btn btn-primary" onclick={onRemove}> Remove photo </button>{' '}
						<button onclick={onClose} class="btn btn-secondary"> Cancel </button>
					</div>
				</div>
			</div>
		{/if}
	{/if}
</div>
