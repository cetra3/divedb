<script lang="ts">
	import SearchIcon from '#lib/icons/SearchIcon.svelte';
	import { client } from '#lib/graphql/client.js';
	import type { SearchResultNodeFragment } from '#lib/graphql/generated.js';
	type CategoryMap = { [category: string]: string[] };

	import SearchResult from './SearchResult.svelte';
	import { onMount, untrack } from 'svelte';
	import { throttle } from 'lodash-es';

	interface Props {
		query?: string;
		map?: CategoryMap | undefined;
		filter?: string | undefined;
		showSearchBar?: boolean;
		title?: string;
		children?: import('svelte').Snippet;
	}

	let {
		query = $bindable(''),
		map = undefined,
		filter = undefined,
		showSearchBar = true,
		title = 'Search DiveDB',
		children
	}: Props = $props();

	let scrollPercent = 0;
	let browserLoaded = $state(false);
	let atTheEnd = false;

	onMount(() => {
		browserLoaded = true;
	});

	let results: SearchResultNodeFragment[] = $state([]);

	let loading = $state(false);
	let called = $state(false);
	let offset = 0;

	let hasValues = $derived(map !== undefined && Object.values(map).flat().length > 0);

	// Incremented for each search, so responses to superseded searches can be ignored
	let latestSearch = 0;

	const updateResult = (more: boolean = false) => {
		// Loading more waits for the current request, but a new query always searches
		if ((query == '' && !hasValues) || (more && loading)) {
			return;
		}

		const search = ++latestSearch;
		loading = true;

		if (!more) {
			offset = 0;
			atTheEnd = false;
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

		client
			.search({ query: queryString, offset })
			.then((val) => {
				if (search != latestSearch) {
					return;
				}
				called = true;
				results = more ? [...results, ...val.search] : val.search;
				offset = results.length;
				atTheEnd = val.search.length == 0;
			})
			.finally(() => {
				if (search == latestSearch) {
					loading = false;
				}
			});
	};

	// Search again only when the query or category filters change, not on anything updateResult reads
	$effect(() => {
		query;
		map;
		untrack(() => updateResult());
	});

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

<svelte:window onscroll={handleScroll} />
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
	{@render children?.()}
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
