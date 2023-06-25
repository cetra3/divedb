<script lang="ts">
	import { browser } from '$app/environment';
	import DiveSiteIcon from '$lib/icons/DiveSiteIcon.svelte';
	import type { RegionNodeFragment } from '$lib/graphql/generated';
	import type { PageData } from './$types';
	import type { LatLngBoundsExpression } from 'leaflet';
	export let data: PageData;
	let diveSites = data.diveSites;

	import { page } from '$app/stores';

	import { onMount } from 'svelte';

	let DiveMap: any;

	const site = browser ? $page.url.searchParams.get('site') : undefined;

	let region: RegionNodeFragment | undefined = undefined;
	let bounds: LatLngBoundsExpression | undefined = undefined;

	$: {
		if (data.region) {
			region = data.regions.find((val) => val.slug == data.region);

			if (region) {
				bounds = [
					[region.latMin, region.lonMin],
					[region.latMax, region.lonMax]
				];
			}
		} else {
			region = undefined;
			bounds = undefined;
		}
	}

	onMount(async () => {
		if (browser) {
			DiveMap = (await import('$lib/components/DiveMap.svelte')).default;
		}
	});
</script>

<svelte:head>
	<title>DiveDB - Dive Map</title>
</svelte:head>

<div class="dive-sites container grid-lg">
	<div class="columns">
		<div class="column col-12 col-lg-12">
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
	</div>
	<div class="columns dive-site-content">
		{#if browser}
			<svelte:component this={DiveMap} sites={diveSites} selectedSite={site} {bounds} />
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
	}
</style>
