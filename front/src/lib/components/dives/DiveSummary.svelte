<script lang="ts">
	import type { DiveWithMetricsFragment } from '$lib/graphql/generated';
	import formatMinutes from '$lib/util/formatMinutes';
	import ImageList from '../ImageList.svelte';
	import DiveGraph from './DiveGraph.svelte';
	import DiveLabels from './DiveLabels.svelte';

	export let dive: DiveWithMetricsFragment;

	$: title = `${dive.user.displayName ?? '@' + dive.user.username} - #${dive.number} ${
		dive.diveSite ? ` - ${dive.diveSite.name}` : ''
	}`;
</script>

<div class="column col-6 col-lg-12">
	<div class="card">
		<DiveGraph {dive} link={true} smallOnly={true} />
		<div class="card-header">
			<div class="card-title h4">
				<a href={`/dives/${dive.id}`}>
					{title}
				</a>
			</div>
			<div class="card-subtitle title-center" />
			<div class="card-subtitle title-center">
				Max Depth: {dive.depth.toFixed(2)}m, Duration: {formatMinutes(dive.duration)}
			</div>
		</div>
		{#if dive.summary != ''}
			<div class="card-body">
				{dive.summary}
			</div>
		{/if}
		<div class="card-footer no-padding">
			{#if dive.latestPhotos.length > 0}
				<ImageList photos={dive.latestPhotos} condensed />
			{/if}
			<DiveLabels {dive} />
		</div>
	</div>
</div>

<style global lang="scss">
	.dive-photo {
		&.more {
			font-size: 1rem;
			font-weight: bold;
			display: flex;
			align-items: center;
			justify-content: center;
			flex-direction: column;
		}

		margin-bottom: 0.8rem;
	}
</style>
