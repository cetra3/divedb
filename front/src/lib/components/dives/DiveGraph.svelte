<script lang="ts">
	import type { DiveWithMetricsFragment, DiveNodeFragment } from '$lib/graphql/generated';
	import GraphImage from './GraphImage.svelte';
	import GraphPlaceholder from './GraphPlaceholder.svelte';


	interface Props {
		link?: boolean;
		smallOnly?: boolean;
		dive: DiveWithMetricsFragment | DiveNodeFragment;
	}

	let { link = false, smallOnly = false, dive }: Props = $props();
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
