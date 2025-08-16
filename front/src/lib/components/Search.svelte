<script lang="ts">
	import { run } from 'svelte/legacy';

	import SearchIcon from '$lib/icons/SearchIcon.svelte';
	import { client } from '$lib/graphql/client';
	import type { SearchResultNodeFragment } from '$lib/graphql/generated';
	type CategoryMap = { [category: string]: string[] };

	import SearchResult from './SearchResult.svelte';
	import { onMount } from 'svelte';
	import { result, throttle } from 'lodash-es';


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

	run(() => {
		(query, map, updateResult());
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
