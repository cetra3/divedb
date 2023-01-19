<script type="ts">
	import { client } from '$lib/graphql/client';
	import type { SearchResultNodeFragment } from '$lib/graphql/generated';
	import { onMount } from 'svelte';

	let focused = false;
	let dropDownFocused = false;

	export let id: string | undefined = undefined;
	let query = '';
	let sealifeLoading = false;
	let loading = false;

	let idx: number | undefined = 0;
	let input: HTMLElement | undefined = undefined;

	let results: SearchResultNodeFragment[] = [];

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

	$: query, focused, updateResult();
</script>

<div class="has-icon-right">
	<input
		bind:this={input}
		class="form-input"
		type="text"
		placeholder="Start typing to find sealife..."
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

	{#if sealifeLoading || loading}
		<i class="form-icon loading" />
	{/if}
</div>

{#if results.length > 0 && (dropDownFocused || focused)}
	<span
		on:mouseenter={() => (dropDownFocused = true)}
		on:mouseleave={() => (dropDownFocused = false)}
		class="dropdown"
		class:active={focused}
	>
		<ul class="menu">
			{#each results as result, index}
				<li class="menu-item">
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
