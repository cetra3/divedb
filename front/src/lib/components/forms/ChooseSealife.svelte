<script lang="ts">
	import { run } from 'svelte/legacy';

	import { client } from '$lib/graphql/client';
	import type { SearchResultNodeFragment } from '$lib/graphql/generated';
	import { onMount } from 'svelte';
	import Photo from '../photos/Photo.svelte';
	import SealifeSummary from '../SealifeSummary.svelte';
	import SearchResult from '../SearchResult.svelte';

	let focused = $state(false);
	let dropDownFocused = $state(false);

	interface Props {
		id?: string | undefined;
	}

	let { id = $bindable(undefined) }: Props = $props();
	let query = $state('');
	let sealifeLoading = $state(false);
	let loading = $state(false);

	let idx: number | undefined = $state(0);
	let input: HTMLElement | undefined = $state(undefined);

	let results: SearchResultNodeFragment[] = $state([]);

	onMount(() => {
		if (id) {
			sealifeLoading = true;
			client.getSealife({ id }).then((res) => {
				const sealife = res.sealife[0];
				query = sealife.name;
				sealifeLoading = false;
			});
		}
	});
	const updateResult = () => {
		if (query != '' && focused) {
			loading = true;

			client.search({ query: `${query} kind:sealife` }).then((val) => {
				loading = false;
				results = val.search;
				idx = undefined;
			});
		}
	};

	run(() => {
		(query, focused, updateResult());
	});
</script>

<div class="has-icon-right">
	<input
		bind:this={input}
		class="form-input"
		type="text"
		placeholder="Start typing to find sealife..."
		onfocus={() => (focused = true)}
		onblur={() => (focused = false)}
		onkeydown={(e) => {
			if (results.length > 0) {
				let curIdx = idx ?? 0;
				if (e.key == 'ArrowDown') {
					e.preventDefault();
					if (idx == undefined) {
						idx = 0;
					} else if (results.length > curIdx + 1) {
						idx = curIdx + 1;
					}

					document.getElementById(`sealife-list-item-${idx}`)?.scrollIntoView({
						block: 'center'
					});
				}
				if (e.key == 'ArrowUp') {
					e.preventDefault();
					if (curIdx > 0) {
						idx = curIdx - 1;
					}

					document.getElementById(`sealife-list-item-${idx}`)?.scrollIntoView({
						block: 'center'
					});
				}
				if (e.key == 'Enter') {
					e.preventDefault();
					let result = results[curIdx];

					query = result.name;
					id = result.id;
					dropDownFocused = false;

					if (input) {
						input.blur();
					}
				}
			}
		}}
		bind:value={query}
	/>

	{#if sealifeLoading || loading}
		<i class="form-icon loading"></i>
	{/if}
</div>

{#if results.length > 0 && (dropDownFocused || focused)}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<span
		onmouseenter={() => (dropDownFocused = true)}
		onmouseleave={() => (dropDownFocused = false)}
		class="dropdown"
		class:active={focused}
	>
		<div class="menu sealife-menu">
			{#each results as result, index}
				<!-- svelte-ignore a11y_invalid_attribute -->
				<a
					class="sealife-item"
					class:active={index == idx}
					id={`sealife-list-item-${index}`}
					href="javascript:void(0)"
					onclick={() => {
						query = result.name;
						id = result.id;

						focused = false;
						dropDownFocused = false;
					}}
				>
					<!-- svelte-ignore a11y_invalid_attribute -->
					<div class="sealife-photo">
						{#if result.photoId}
							<Photo id={result.photoId} />
						{/if}
					</div>
					<div class="sealife-text">
						<strong>{result.name}</strong>
						{#if result.scientificName}
							{result.scientificName}
						{/if}
					</div>
				</a>
			{/each}
		</div>
	</span>
{/if}

<style lang="scss" global>
	.sealife-menu {
		bottom: 2.4rem;
		top: auto !important;
		left: 0;
		right: 0;
		max-height: 15rem !important;
	}

	.sealife-item {
		display: flex;
		&:not(:last-child) {
			margin-bottom: 0.2rem;
		}

		border-radius: 0.3rem;
		border: 0.05rem solid #dce2ec;

		&:hover,
		&.active {
			background-color: #5dc5ed;
			text-decoration: none;
		}
	}

	.sealife-text {
		display: flex;
		flex-direction: column;
		padding: 0.3rem;
		flex: 1;
	}

	.sealife-photo {
		img {
			border-radius: 0.3rem 0 0 0.3rem;
			width: 4rem;
			min-height: 3rem;
			object-fit: cover;
		}
	}
</style>
