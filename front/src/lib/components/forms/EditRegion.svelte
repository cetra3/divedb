<script lang="ts">
	import type { CreateRegion, RegionNodeFragment } from '$lib/graphql/generated';
	import FormRow from '../FormRow.svelte';
	export let region: CreateRegion | RegionNodeFragment | undefined = undefined;
	export let onSave: (region: CreateRegion) => void;
	import { session } from '$lib/session';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import type { LatLngBoundsExpression, LatLngLiteral } from 'leaflet';
	import { fromLeaflet } from '$lib/util/bounds';

	let name = region?.name ?? '';

	let bounds: LatLngBoundsExpression | undefined = region
		? [
				[region.latMin, region.lonMin],
				[region.latMax, region.lonMax]
			]
		: undefined;

	$: canSave = name != '' && bounds !== undefined;

	$: isEditor = $session.user?.level == 'ADMIN' || $session.user?.level == 'EDITOR';

	const onSubmit = (e: Event) => {
		e.preventDefault();

		if (bounds !== undefined) {
			let { latMin, lonMin, latMax, lonMax } = fromLeaflet(bounds);

			onSave({
				id: region?.id,
				name,
				latMin,
				lonMin,
				latMax,
				lonMax
			});
		}
	};

	let EditableRegion: any;

	onMount(async () => {
		if (browser) {
			EditableRegion = (await import('$lib/components/EditableRegion.svelte')).default;
		}
	});
</script>

<div class="columns">
	<div class="column col-12 col-sm-12">
		<form class="form-horizontal" on:submit={onSubmit}>
			<FormRow name="Name">
				<input type="text" placeholder="Name" bind:value={name} class="form-input" />
			</FormRow>

			<FormRow name="Region Bounds">
				{#if browser}
					<svelte:component this={EditableRegion} bind:bounds />
				{/if}
			</FormRow>

			<FormRow name="Submit">
				<button class="btn btn-primary" type="submit" disabled={canSave == false}
					>Submit Region</button
				>
			</FormRow>
		</form>
	</div>
</div>
