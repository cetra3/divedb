<script lang="ts">
	import type { CreatePhoto, PhotoSummaryFragment } from '$lib/graphql/generated';
	import FormRow from '../FormRow.svelte';
	import ChooseSealife from './ChooseSealife.svelte';
	import ChooseSite from './ChooseSite.svelte';
	interface Props {
		photo: CreatePhoto | PhotoSummaryFragment;
		onSave: (photo: CreatePhoto) => void;
	}

	let { photo, onSave }: Props = $props();

	let diveSiteId: string | undefined = $state();

	if ('diveSite' in photo) {
		diveSiteId = photo.diveSite?.id;
	} else if ('diveSiteId' in photo) {
		diveSiteId = photo.diveSiteId ?? undefined;
	}

	let sealifeId: string | undefined = $state();

	if ('sealife' in photo) {
		sealifeId = photo.sealife?.id;
	} else if ('sealifeId' in photo) {
		sealifeId = photo.sealifeId ?? undefined;
	}

	const onSubmit = (e: Event) => {
		e.preventDefault();
		let photoUpdate: CreatePhoto = {
			id: photo.id,
			userId: photo.userId,
			date: photo.date,
			size: photo.size,
			filename: photo.filename,
			sealifeId: sealifeId ?? null,
			diveSiteId: diveSiteId ?? null
		};
		onSave(photoUpdate);
	};
</script>

<form class="form-horizontal" onsubmit={onSubmit}>
	<FormRow name="Sealife">
		<ChooseSealife bind:id={sealifeId} />
	</FormRow>
	<FormRow name="Dive Site">
		<ChooseSite bind:id={diveSiteId} />
	</FormRow>
	<FormRow name="">
		<button class="btn btn-primary" type="submit">Submit Photo</button>
	</FormRow>
</form>
