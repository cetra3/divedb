<script type="ts">
	import DiveSummary from '$lib/components/dives/DiveSummary.svelte';
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
	let dives = data.dives;
	let width: number;
</script>

<svelte:head>
	<title>DiveDB - Dives</title>
</svelte:head>
<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-lg-12">
			<h1 class="page-title">
				<DiveLogIcon size="1.4em" />
				Your Dives

				<a href="/dives/new">
					<button class="btn btn-secondary btn-sm">Add New</button>
				</a>
				<a href="/dives/subsurface">
					<button class="btn btn-secondary btn-sm">Sync from Subsurface</button>
				</a>
			</h1>
		</div>
		<div bind:clientWidth={width} class="column col-6 col-lg-12" />
	</div>
	{#if dives !== undefined}
		<div class="columns">
			{#each dives as dive, index}
				<DiveSummary {dive} {width} diveNumber={dives.length - index} />
			{/each}
		</div>
	{:else}
		<div class="column col-12">
			<div class="loading loading-lg" />
		</div>
	{/if}
</div>
