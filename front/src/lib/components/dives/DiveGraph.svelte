<script lang="ts">
	import type { DiveWithMetricsFragment, DiveNodeFragment } from '$lib/graphql/generated';
	import GraphImage from './GraphImage.svelte';
	import GraphPlaceholder from './GraphPlaceholder.svelte';

	export let link = false;
	export let smallOnly = false;

	export let dive: DiveWithMetricsFragment | DiveNodeFragment;
</script>

{#if dive.hasMetrics}
	<div class="card-image">
		{#if link}
			<a href={`/dives/${dive.id}`}>
				<GraphImage diveId={dive.id} {smallOnly} />
			</a>
		{:else}
			<GraphImage diveId={dive.id} {smallOnly} />
		{/if}
	</div>
{:else if link}
	<a href={`/dives/${dive.id}`}>
		<GraphPlaceholder />
	</a>
{:else}
	<GraphPlaceholder />
{/if}
