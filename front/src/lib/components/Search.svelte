<script lang="ts">
	import SearchIcon from '$lib/icons/SearchIcon.svelte';
	import { client } from '$lib/graphql/client';
	import type { SearchResultNodeFragment } from '$lib/graphql/generated';
	type CategoryMap = { [category: string]: string[] };

	import SearchResult from './SearchResult.svelte';
	import { onMount } from 'svelte';
	import { result, throttle } from 'lodash-es';

	export let query = '';
	export let map: CategoryMap | undefined = undefined;
	export let filter: string | undefined = undefined;
	export let showSearchBar = true;

	export let title = 'Search DiveDB';

	let scrollPercent = 0;
	let browserLoaded = false;
	let atTheEnd = false;

	onMount(() => {
		browserLoaded = true;
	});

	let results: SearchResultNodeFragment[] = [];

	let loading = false;
	let called = false;
	let offset = 0;

	$: hasValues = map !== undefined && Object.values(map).flat().length > 0;

	const updateResult = (more: boolean = false) => {
		if ((query != '' || hasValues) && !loading) {
			loading = true;

			if (!more) {
				offset = 0;
			}

			let queryString = query;

			if (filter) {
				queryString += ` ${filter} `;
			}

			if (map) {
				queryString += Object.values(map)
					.flat()
					.map((val) => `category:${val}`)
					.join(' ');
			}

			client.search({ query: queryString, offset }).then((val) => {
				loading = false;
				called = true;
				results = more ? [...results, ...val.search] : val.search;
				offset = results.length;
				atTheEnd = val.search.length == 0;
			});
		}
	};

	$: (query, map, updateResult());

	const handleScroll = throttle(() => {
		let scrollTop = window.scrollY;
		let docHeight = document.body.offsetHeight;
		let winHeight = window.innerHeight;
		scrollPercent = scrollTop / (docHeight - winHeight);

		if (scrollPercent > 0.5 && !loading && !atTheEnd) {
			updateResult(true);
		}
	}, 300);
</script>

<svelte:window on:scroll={handleScroll} />
<div class="container grid-lg">
	{#if showSearchBar}
		<div class="columns">
			<div class="column col-12 search-row">
				<SearchIcon size="1.4em" />
				<input
					disabled={browserLoaded === false}
					class={'form-input search-bar'}
					type="text"
					placeholder={title}
					bind:value={query}
				/>
			</div>
		</div>
	{/if}
	<slot />
	<div class="columns">
		{#if query != '' || hasValues}
			{#if loading && !called}
				<div class="column col-12">
					<div class="loading loading-lg"></div>
				</div>
			{:else}
				{#if results.length === 0}
					<div class="column col-12">
						<p>
							{#if query !== ''}
								No results found for "{query}", try another search!
							{/if}
							{#if hasValues}
								No results found for those categories, try another search!
							{/if}
						</p>
					</div>
				{/if}
				{#each results as result}
					<SearchResult {result} />
				{/each}
			{/if}
		{/if}
	</div>
</div>

<style lang="scss" global>
	.search-row {
		display: flex;
		svg {
			margin: 7px;
		}
	}
</style>
