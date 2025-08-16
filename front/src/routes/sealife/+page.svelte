<script lang="ts">
	import type { PageData } from './$types';
	import Search from '$lib/components/Search.svelte';
	import SealifeSummary from '$lib/components/SealifeSummary.svelte';
	import SealifeIcon from '$lib/icons/SealifeIcon.svelte';
	import CategoryPicker from '$lib/components/categories/CategoryPicker.svelte';
	import { categoryStore } from '$lib/category';
	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();
	let sealife = $derived(data.sealife);
	let categories = $derived(data.categories);

	let hasValues = $derived(Object.values($categoryStore).flat().length > 0);

	let query: string = $state();
	let showSearch = $state(false);
</script>

<svelte:head>
	<title>DiveDB - Sealife</title>
</svelte:head>

<div class="dive-sites container grid-lg">
	<div class="columns">
		<div class="column col-12 col-lg-12">
			<h1 class="page-title">
				<SealifeIcon size="22px" /> Sealife
				<button
					onclick={() => {
						showSearch = !showSearch;
					}}
					class:btn-primary={showSearch}
					class:btn-secondary={!showSearch}
					class="btn btn-sm">Search</button
				>
				<a href="/sealife/new">
					<button class="btn btn-secondary btn-sm">Add New</button>
				</a>
			</h1>
		</div>
	</div>
</div>

{#if showSearch || hasValues}
	<Search
		showSearchBar={showSearch}
		bind:query
		map={$categoryStore}
		filter="kind:sealife"
		title="Search Sealife"
	>
		<div class="columns">
			<div class="column col-12 col-lg-12">
				<CategoryPicker {categories} bind:map={$categoryStore} />
			</div>
		</div>
	</Search>
{:else}
	<div class="container grid-lg">
		<div class="columns">
			<div class="column col-12 col-lg-12">
				<CategoryPicker {categories} bind:map={$categoryStore} />
			</div>
		</div>
	</div>
{/if}

{#if (!showSearch || query == undefined || query == '') && !hasValues}
	<div class="dive-sites container grid-lg">
		<div class="columns">
			{#if sealife !== undefined}
				{#each sealife as val}
					<SealifeSummary sealife={val} />
				{/each}
			{/if}
		</div>
	</div>
{/if}
