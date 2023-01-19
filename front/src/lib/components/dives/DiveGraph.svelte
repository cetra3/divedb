<script lang="ts">
	import type { DiveWithMetricsFragment } from '$lib/graphql/generated';
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';

	export let width = 470;
	export let height = 300;
	export let dive: DiveWithMetricsFragment;
</script>

{#if dive.hasMetrics}
	<div class="card-image">
		<img
			loading="lazy"
			{width}
			{height}
			src={`/api/chart/${dive.id}?width=${width}&height=${height}`}
			class="img-responsive"
			alt={'Dive Chart'}
		/>
	</div>
{:else}
	<div class="hero bg-dark flex-center">
		<div class="hero-body">
			<DiveLogIcon size="88px" />
		</div>
	</div>
{/if}
