<script type="ts">
	import DiveSiteSummary from '$lib/components/DiveSiteSummary.svelte';
	import DiveSiteIcon from '$lib/icons/DiveSiteIcon.svelte';
	import Search from '$lib/components/Search.svelte';
	import type { PageData } from './$types';
	export let data: PageData;
	let diveSites = data.diveSites;

	let query: string;
	let showSearch = false;
</script>

<svelte:head>
	<title>DiveDB - Dive Sites</title>
</svelte:head>

<div class="dive-sites container grid-lg">
	<div class="columns">
		<div class="column col-6 col-lg-12">
			<h1 class="page-title">
				<DiveSiteIcon size="1em" /> Dive Sites
				<button
					on:click={() => {
						showSearch = !showSearch;
					}}
					class:btn-primary={showSearch}
					class:btn-secondary={!showSearch}
					class="btn btn-sm">Search</button
				>
				<a href="/divesites/new">
					<button class="btn btn-secondary btn-sm">Add New</button>
				</a>
			</h1>
		</div>
		<div class="column col-3 col-ml-auto col-lg-12">
			<ul class="page-title tab tab-block">
				<li class="tab-item">
					<a class="active" href="/divesites"> List </a>
				</li>
				<li class="tab-item">
					<a href="/divesites/map"> Map </a>
				</li>
			</ul>
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
