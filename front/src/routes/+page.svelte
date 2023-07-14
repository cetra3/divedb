<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
	import DiveSiteSummary from '$lib/components/DiveSiteSummary.svelte';
	import DiveSiteIcon from '$lib/icons/DiveSiteIcon.svelte';
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import Search from '$lib/components/Search.svelte';
	import { assets } from '$app/paths';
	import DiveSummary from '$lib/components/dives/DiveSummary.svelte';

	$: diveSites = data.frontPage.popularDiveSites;
	$: recentDives = data.frontPage.recentDives;

	let query: string;
</script>

<svelte:head>
	<title>DiveDB</title>
	<meta property="og:type" content="website" />
	<meta
		name="description"
		property="og:description"
		content="Scuba Diving Database with Dive Sites and Dive Photos"
	/>
	<meta property="og:site_name" content="DiveDB" />
	<meta property="og:image" content="{assets}/logo.png" />
</svelte:head>

<Search bind:query />

{#if query == ''}
	<div class="container grid-lg">
		<div class="columns">
			<div class="column col-12 col-sm-12">
				<h1 class="page-title">
					<DiveLogIcon size="1.4em" />
					Recent Dives
					<a href="/dives">
						<button class="btn btn-primary btn-sm"> View Dives </button>
					</a>
					<a href="/dives/new">
						<button class="btn btn-sm"> Add New </button>
					</a>
				</h1>
			</div>
			{#each recentDives as dive}
				<DiveSummary {dive} />
			{/each}

			<div class="column col-12 col-sm-12">
				<h1 class="page-title">
					<DiveSiteIcon size="1.4em" />
					Popular Sites
					<a href="/divesites/map">
						<button class="btn btn-primary btn-sm"> View Map </button>
					</a>
					<a href="/divesites">
						<button class="btn btn-sm"> View List </button>
					</a>
				</h1>
			</div>

			{#each diveSites as site}
				<DiveSiteSummary {site} />
			{/each}
		</div>
	</div>
{/if}
