<script lang="ts">
	import { browser } from '$app/env';
	import DiveSiteIcon from '#lib/icons/DiveSiteIcon.svelte';
	import type { RegionNodeFragment } from '#lib/graphql/generated.js';
	import type { PageData } from './$types';
	import type { LatLngBoundsExpression } from 'leaflet';

	import { page } from '$app/state';

	import { onMount } from 'svelte';
	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();

	let DiveMap: any = $state();

	const site = browser ? page.url.searchParams.get('site') : undefined;

	let region: RegionNodeFragment | undefined = $derived(
		data.region ? data.regions.find((val) => val.slug == data.region) : undefined
	);
	let bounds: LatLngBoundsExpression | undefined = $derived(
		region
			? [
					[region.latMin, region.lonMin],
					[region.latMax, region.lonMax]
				]
			: undefined
	);

	onMount(async () => {
		if (browser) {
			DiveMap = (await import('#lib/components/DiveMap.svelte')).default;
		}
	});
</script>

<svelte:head>
	<title>DiveDB - Dive Map</title>
</svelte:head>

<div class="dive-sites">
	<div class="container grid-lg">
		<h1 class="page-title inline">
			<DiveSiteIcon size="22px" /> Dive Map
			{#if data.regions.length > 0}
				<div class="dropdown inline">
					<button class="btn btn-sm dropdown-toggle">
						{#if region != undefined}
							{region.name}
						{:else}
							Choose a Region
						{/if}
					</button>
					<ul class="menu region-list">
						{#each data.regions as region}
							<li class="menu-item">
								<a href="/divesites/map/{region.slug}">{region.name}</a>
							</li>
						{/each}
					</ul>
				</div>
			{/if}
			<a href="/divesites">
				<button class="btn btn-secondary btn-sm">View List</button>
			</a>
		</h1>
	</div>
	<div class="dive-site-content">
		{#if browser}
			{#await data.diveSites}
				<div class="column col-12 map-loading"><div class="loading loading-lg"></div></div>
			{:then diveSites}
				<DiveMap sites={diveSites} selectedSite={site} {bounds} />
			{:catch}
				<div class="column col-12 map-loading">Could not load dive sites</div>
			{/await}
		{/if}
	</div>
</div>

<style>
	.inline {
		display: inline-block;
		margin-bottom: 0.1rem;
	}
	.dive-sites {
		flex: 1;
		display: flex;
		flex-direction: column;
	}

	.region-list {
		z-index: 1000;
		font-size: 0.8rem;
		font-weight: normal;
	}
	.dive-site-content {
		flex: 1;
		display: flex;
	}
	.dive-site-content :global(.leaflet-container) {
		border-radius: 0.3rem;
	}
	.map-loading {
		display: flex;
		align-items: center;
		justify-content: center;
	}
</style>
