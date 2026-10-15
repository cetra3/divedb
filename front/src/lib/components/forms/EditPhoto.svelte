<script lang="ts">
	import type { CreatePhoto, PhotoSummaryFragment } from '#lib/graphql/generated.js';
	import FormRow from '../FormRow.svelte';
	import ChooseSealife from './ChooseSealife.svelte';
	import ChooseSite from './ChooseSite.svelte';
	interface Props {
		photo: CreatePhoto | PhotoSummaryFragment;
		onSave: (photo: CreatePhoto) => void;
	}

	let { photo, onSave }: Props = $props();

	// Editable via the pickers, and reset when a different photo is passed in
	let diveSiteId: string | undefined = $derived(
		'diveSite' in photo
			? photo.diveSite?.id
			: 'diveSiteId' in photo
				? (photo.diveSiteId ?? undefined)
				: undefined
	);
	let sealifeId: string | undefined = $derived(
		'sealife' in photo
			? photo.sealife?.id
			: 'sealifeId' in photo
				? (photo.sealifeId ?? undefined)
				: undefined
	);

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
