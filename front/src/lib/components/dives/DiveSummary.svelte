<script lang="ts">
	import type { DiveWithMetricsFragment } from '$lib/graphql/generated';
	import formatMinutes from '$lib/util/formatMinutes';
	import DiveGraph from './DiveGraph.svelte';

	export let width = 470;
	export let diveNumber: number;
	export let dive: DiveWithMetricsFragment;

	$: title = `#${diveNumber}${dive.diveSite ? ` - ${dive.diveSite.name}` : ''}`;
</script>

<div class="column col-6 col-lg-12">
	<div class="card">
		<a href={`/dives/${dive.id}`}>
			<DiveGraph {width} {dive} />
		</a>

		<div class="card-header">
			<div class="card-title h4">
				<a href={`/dives/${dive.id}`}>
					{title}
				</a>
			</div>
			<div class="card-subtitle title-center">
				Max Depth: {dive.depth.toFixed(2)}m, Time: {formatMinutes(dive.duration)}
			</div>
		</div>
		<div class="card-footer">
			{#if dive.diveSite}
				<a href={`/sites/${dive.diveSite.slug}`}>
					<span class="label label-primary">{dive.diveSite.name}</span>
				</a>
			{/if}
			{#if dive.date}
				<span class="label">{new Date(dive.date).toLocaleString()}</span>
			{/if}
		</div>
	</div>
</div>
