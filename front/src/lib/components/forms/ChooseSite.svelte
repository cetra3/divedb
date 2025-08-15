<script lang="ts">
	import { client } from '$lib/graphql/client';
	import type { SearchResultNodeFragment } from '$lib/graphql/generated';
	import { onMount } from 'svelte';

	let focused = false;
	let dropDownFocused = false;

	export let belowInput = false;

	export let id: string | null | undefined = undefined;
	let query = '';
	let siteLoading = false;
	let loading = false;

	let idx: number | undefined = 0;
	let input: HTMLInputElement | undefined = undefined;

	let results: SearchResultNodeFragment[] = [];

	onMount(() => {
		if (id) {
			siteLoading = true;
			client.getDiveSites({ id }).then((res) => {
				const diveSite = res.diveSites[0];
				query = diveSite.name;
				siteLoading = false;
			});
		}
	});
	const updateResult = () => {
		if (query != '' && focused) {
			loading = true;

			client.search({ query: `${query} kind:dive_site` }).then((val) => {
				loading = false;
				results = val.search;
				idx = undefined;
			});
		}
	};

	$: (query, focused, updateResult());
</script>

<div class="has-icon-right">
	<input
		bind:this={input}
		class="form-input"
		type="text"
		placeholder="Start typing to find a site..."
		on:focus={() => (focused = true)}
		on:blur={() => (focused = false)}
		on:keydown={(e) => {
			if (results.length > 0) {
				let curIdx = idx ?? 0;
				if (e.key == 'ArrowDown') {
					e.preventDefault();
					if (idx == undefined) {
						idx = 0;
					} else if (results.length > curIdx + 1) {
						idx = curIdx + 1;
					}
				}
				if (e.key == 'ArrowUp') {
					e.preventDefault();
					if (curIdx > 0) {
						idx = curIdx - 1;
					}
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

	{#if siteLoading || loading}
		<i class="form-icon loading"></i>
	{/if}
</div>

{#if results.length > 0 && (dropDownFocused || focused)}
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<span
		on:mouseenter={() => (dropDownFocused = true)}
		on:mouseleave={() => (dropDownFocused = false)}
		class="dropdown"
		class:active={focused}
	>
		<ul class="menu" class:above-input={belowInput == false}>
			{#each results as result, index}
				<li class="menu-item">
					<!-- svelte-ignore a11y-invalid-attribute -->
					<a
						class:active={index == idx}
						href="javascript:void(0)"
						on:click={() => {
							query = result.name;
							id = result.id;

							focused = false;
							dropDownFocused = false;
						}}>{result.name}</a
					>
				</li>
			{/each}
		</ul>
	</span>
{/if}

<style lang="scss">
	.above-input {
		bottom: 2.4rem;
		top: auto !important;
		left: 0;
		right: 0;
		max-height: 15rem !important;
	}
</style>
