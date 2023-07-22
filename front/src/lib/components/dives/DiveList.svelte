<script lang="ts">
	import type {
		DiveSummaryFragment,
		DiveWithMetricsFragment,
		GetDivesQueryVariables
	} from '$lib/graphql/generated';
	import { client } from '$lib/graphql/client';
	import { throttle } from 'lodash-es';
	import DiveSummary from './DiveSummary.svelte';
	export let query: GetDivesQueryVariables | undefined = undefined;
	export let dives: DiveWithMetricsFragment[];

	let scrollPercent = 0;
	let loading = false;
	let atTheEnd = false;

	$: offset = dives.length;

	const moreDives = () => {
		if (query !== undefined && !loading && !atTheEnd) {
			loading = true;
			client.getDives({ ...query, offset }).then((val) => {
				atTheEnd = val.dives.length == 0;
				dives = [...dives, ...val.dives];
				loading = false;
			});
		}
	};

	const handleScroll = throttle(() => {
		let scrollTop = window.scrollY;
		let docHeight = document.body.offsetHeight;
		let winHeight = window.innerHeight;
		scrollPercent = scrollTop / (docHeight - winHeight);

		if (scrollPercent > 0.5) {
			moreDives();
		}
	}, 300);
</script>

<svelte:window on:scroll={handleScroll} />
<div class="columns">
	{#each dives as dive}
		<DiveSummary {dive} />
	{/each}
</div>
