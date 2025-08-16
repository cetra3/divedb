<script lang="ts">
	import DiveSiteSummary from '$lib/components/DiveSiteSummary.svelte';
	import DiveSiteIcon from '$lib/icons/DiveSiteIcon.svelte';
	import Search from '$lib/components/Search.svelte';
	import type { PageData } from './$types';
	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();
	let diveSites = data.diveSites;

	let query: string = $state();
	let showSearch = $state(false);
</script>

<svelte:head>
	<title>DiveDB - Dive Sites</title>
</svelte:head>

<div class="dive-sites container grid-lg">
	<div class="columns">
		<div class="column col-12 col-lg-12">
			<h1 class="page-title">
				<DiveSiteIcon size="1em" /> Dive Sites
				<button
					onclick={() => {
						showSearch = !showSearch;
					}}
					class:btn-primary={showSearch}
					class:btn-secondary={!showSearch}
					class="btn btn-sm">Search</button
				>
				<a href="/divesites/new">
					<button class="btn btn-secondary btn-sm">Add New</button>
				</a>
				<a href="/divesites/map">
					<button class="btn btn-secondary btn-sm">View Map</button>
				</a>
			</h1>
		</div>
	</div>
</div>
{#if showSearch}
	<Search bind:query filter="kind:dive_site" title="Search Dive Sites" />
{/if}

{#if !showSearch || query == undefined || query == ''}
	<div class="dive-sites container grid-lg">
		<div class="columns dive-site-content">
			{#each diveSites.diveSites as site}
				<DiveSiteSummary {site} />
			{/each}
		</div>
	</div>
{/if}
