<script type="ts">
	import type { CreatePhoto, PhotoSummaryFragment } from '$lib/graphql/generated';
	import FormRow from '../FormRow.svelte';
	import ChooseSealife from './ChooseSealife.svelte';
	import ChooseSite from './ChooseSite.svelte';
	export let photo: CreatePhoto | PhotoSummaryFragment;
	export let onSave: (photo: CreatePhoto) => void;

	let diveSiteId: string | undefined;

	if ('diveSite' in photo) {
		diveSiteId = photo.diveSite?.id;
	} else if ('diveSiteId' in photo) {
		diveSiteId = photo.diveSiteId;
	}

	let sealifeId: string | undefined;

	if ('sealife' in photo) {
		sealifeId = photo.sealife?.id;
	} else if ('sealifeId' in photo) {
		sealifeId = photo.sealifeId;
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

<form class="form-horizontal" on:submit={onSubmit}>
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
