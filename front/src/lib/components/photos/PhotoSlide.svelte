<script lang="ts">
	import { client } from '$lib/graphql/client';

	import type { PhotoSummaryFragment } from '$lib/graphql/generated';
	import EditPhoto from '../forms/EditPhoto.svelte';


	interface Props {
		photo: PhotoSummaryFragment;
		onEditSave: (fragment: PhotoSummaryFragment) => void;
	}

	let { photo = $bindable(), onEditSave }: Props = $props();
</script>

<div class="edit-form">
	<EditPhoto
		onSave={(newPhoto) => {
			client.updatePhoto({ photo: newPhoto }).then((val) => {
				photo = val.updatePhoto;
				onEditSave(val.updatePhoto);
			});
		}}
		{photo}
	/>
</div>

<style lang="scss">
	.edit-form {
		padding: 0.8rem;
	}
</style>
