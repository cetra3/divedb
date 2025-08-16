<script lang="ts">
	import type { Category, CreateSealife, SealifeNodeFragment } from '$lib/graphql/generated';
	import FormRow from '../FormRow.svelte';
	import { session } from '$lib/session';
	import CategoryPicker from '../categories/CategoryPicker.svelte';
	import Markdown from './Markdown.svelte';
	interface Props {
		sealife: SealifeNodeFragment | CreateSealife;
		categories: Category[];
		onSave: (site: CreateSealife) => void;
	}

	let { sealife = $bindable(), categories, onSave }: Props = $props();

	let categoryMap = $state(sealife.categoryMap ?? {});

	let canSave = $derived(sealife.name != '');

	let isEditor = $derived($session.user?.level == 'ADMIN' || $session.user?.level == 'EDITOR');

	const onSubmit = (e: Event) => {
		e.preventDefault();

		onSave({
			id: sealife.id,
			name: sealife.name,
			scientificName: sealife.scientificName,
			description: sealife.description,
			hideLocation: sealife.hideLocation,
			photoId: sealife.photoId,
			categoryMap
		});
	};
</script>

<div class="columns">
	<div class="column col-12 col-sm-12">
		<form class="form-horizontal" onsubmit={onSubmit}>
			<FormRow name="Name">
				<input type="text" placeholder="Name" bind:value={sealife.name} class="form-input" />
			</FormRow>
			<FormRow name="Scientific Name">
				<input
					type="text"
					placeholder="Scientific Name"
					bind:value={sealife.scientificName}
					class="form-input"
				/>
			</FormRow>
			<FormRow name="Description">
				<Markdown bind:value={sealife.description} />
			</FormRow>
			{#if isEditor}
				<FormRow name="Hide Location">
					<label class="form-switch">
						<input type="checkbox" bind:checked={sealife.hideLocation} />
						<i class="form-icon"></i>Keep locations of sightings hidden (i.e, this is a protected
						species)
					</label>
				</FormRow>
			{/if}
			<FormRow name="Categories">
				<CategoryPicker {categories} bind:map={categoryMap} />
			</FormRow>
			<FormRow name="">
				<button class="btn btn-primary" type="submit" disabled={canSave == false}
					>Submit {sealife.name}</button
				>
			</FormRow>
		</form>
	</div>
</div>
