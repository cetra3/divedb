<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
	import DiveSiteSummary from '$lib/components/DiveSiteSummary.svelte';
	import DiveSiteIcon from '$lib/icons/DiveSiteIcon.svelte';
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import Search from '$lib/components/Search.svelte';
	import { assets } from '$app/paths';

	let diveSites = data.diveSites;

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
					<DiveSiteIcon size="1.4em" />
					Popular Dive Sites
					<a href="/divesites">
						<button class="btn btn-primary btn-sm"> View All Sites </button>
					</a>
				</h1>
			</div>

			{#each diveSites.popularDiveSites as site}
				<DiveSiteSummary {site} />
			{/each}

			<div class="column col-6 col-lg-12">
				<div class="card">
					<div class="card-image">
						<div class="hero bg-dark flex-center">
							<div class="hero-body">
								<DiveLogIcon size="66px" />
							</div>
						</div>
					</div>
					<div class="card-header">
						<div class="card-title h5">Dive Log</div>
						<div class="card-subtitle">Log your dives or sync from subsurface</div>
					</div>
					<div class="card-footer">
						<a href="/dives">
							<button class="btn btn-primary">Open</button>
						</a>
						<a href="/dives/new">
							<button class="btn btn-secondary">Add New</button>
						</a>
					</div>
				</div>
			</div>
			<div class="column col-6 col-lg-12">
				<div class="card">
					<div class="card-image">
						<div class="hero bg-dark flex-center">
							<div class="hero-body">
								<PhotoIcon size="66px" />
							</div>
						</div>
					</div>
					<div class="card-header">
						<div class="card-title h5">Photos</div>
						<div class="card-subtitle ">Add and view photos from your dives</div>
					</div>
					<div class="card-footer">
						<a href="/photos">
							<button class="btn btn-primary">Open</button>
						</a>
						<a href="/photos/new">
							<button class="btn btn-secondary">Add New</button>
						</a>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
