<script type="ts">
	import type { CreateDiveSite, SiteFragment } from '$lib/graphql/generated';
	import FormRow from '../FormRow.svelte';
	import { onMount, SvelteComponent, SvelteComponentTyped } from 'svelte';
	import { browser } from '$app/environment';
	export let site: SiteFragment | CreateDiveSite;
	export let onSave: (site: CreateDiveSite) => void;
	import { session } from '$lib/session';

	let EditableMap: any;

	onMount(async () => {
		if (browser) {
			EditableMap = (await import('$lib/components/EditableMap.svelte')).default;
		}
	});

	const gpsRegex = /^([-+]?\d{1,2}\.\d+)\s*([-+]?\d{1,3}\.\d+)$/;

	let gps = site.lat !== 0 ? `${site.lat} ${site.lon}` : '';

	let showMap = false;

	$: gpsMatch = gps.match(gpsRegex);
	$: gpsError = gps != '' && !gpsMatch;

	$: canSave = site.name != '' && !gpsError;

	$: isEditor =
		$session.user?.level == 'ADMIN' ||
		$session.user?.level == 'EDITOR' ||
		!('userId' in site) ||
		($session.user?.id != undefined && $session.user.id === site.userId);

	const onMapChange = (lat: number, lon: number) => {
		gps = `${lat.toFixed(6)} ${lon.toFixed(6)}`;
	};

	$: if (gpsMatch != undefined) {
		site.lat = +gpsMatch[1];
		site.lon = +gpsMatch[2];
	}

	const onSubmit = (e: Event) => {
		e.preventDefault();

		onSave({
			id: site.id,
			name: site.name,
			description: site.description,
			access: site.access,
			depth: site.depth,
			difficulty: site.difficulty,
			published: site.published,
			lat: site.lat,
			lon: site.lon,
			photoId: site.photoId
		});
	};
</script>

<div class="columns">
	<div class="column col-12 col-sm-12">
		<form class="form-horizontal" on:submit={onSubmit}>
			<FormRow name="Name">
				<input type="text" placeholder="Name" bind:value={site.name} class="form-input" />
			</FormRow>
			<FormRow name="Description">
				<textarea bind:value={site.description} rows="8" class="form-input" />
			</FormRow>
			<FormRow name="Access">
				<textarea bind:value={site.access} rows="8" class="form-input" />
			</FormRow>
			<FormRow name="Depth (m)">
				<input type="number" placeholder="18.0" bind:value={site.depth} class="form-input" />
			</FormRow>
			<FormRow name="Difficulty">
				<label class="form-radio form-inline">
					<input type="radio" bind:group={site.difficulty} value={'OW'} />
					<i class="form-icon" /> Open Water
				</label>
				<label class="form-radio form-inline">
					<input type="radio" bind:group={site.difficulty} value={'AOW'} />
					<i class="form-icon" /> Advanced Open Water
				</label>
				<label class="form-radio form-inline">
					<input type="radio" bind:group={site.difficulty} value={'TECH'} />
					<i class="form-icon" /> Technical
				</label>
			</FormRow>
			<FormRow name="GPS" error={gpsError}>
				<input type="text" placeholder="GPS" bind:value={gps} class="form-input" />
				{#if gpsError}
					<span class="form-input-hint">
						GPS coordinates are in decimal format with no comma i.e,{' '}
						<code>-35.149122 138.46588</code>
						<br />
					</span>
				{/if}
				<span class="form-input-hint">
					<a
						href="javascript:void(0)"
						on:click={() => {
							showMap = !showMap;
						}}
					>
						{showMap ? 'Hide' : 'Show'} map
					</a>
				</span>
			</FormRow>
			{#if showMap}
				<FormRow name="">
					{#if browser}
						<svelte:component
							this={EditableMap}
							lat={site.lat}
							lon={site.lon}
							onChange={onMapChange}
						/>
					{/if}
				</FormRow>
			{/if}
			{#if isEditor}
				<FormRow name="Published">
					<label class="form-switch">
						<input type="checkbox" bind:checked={site.published} />
						<i class="form-icon" /> Publish this dive site (otherwise only you will be able to see it)
					</label>
				</FormRow>
			{/if}
			<FormRow name="">
				<button class="btn btn-primary" type="submit" disabled={canSave == false}
					>Submit {site.name}</button
				>
			</FormRow>
		</form>
	</div>
</div>
