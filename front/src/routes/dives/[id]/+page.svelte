<script lang="ts">
	import DiveGraph from '$lib/components/dives/DiveGraph.svelte';

	import type { PageData } from './$types';

	export let data: PageData;
	import type { DiveNodeFragment } from '$lib/graphql/generated';
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import ImageList from '$lib/components/ImageList.svelte';
	import formatMinutes from '$lib/util/formatMinutes';

	let width: number | undefined;
	let dive = data.dive;

	$: title = dive ? `#${dive.number}${dive.diveSite ? ` - ${dive.diveSite.name}` : ''}` : ' Dive';
</script>

<svelte:head>
	<title>DiveDB - {title}</title>
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		{#if dive !== undefined}
			<div bind:clientWidth={width} class="column col-12 col-sm-12">
				<div class="card">
					<DiveGraph {width} height={width * 0.5} {dive} />
					<div class="card-header">
						<h1 class="page-title">
							<DiveLogIcon size="33px" />
							{title}
							<a href={`/dives/edit/${dive.id}`}>
								<button class="btn btn-secondary btn-sm"> Edit </button>
							</a>
						</h1>
					</div>
					<div class="card-body">
						<div class="columns">
							<div class="column col-6 col-sm-12">
								<h5>Date</h5>
								<p>
									{new Date(dive.date).toLocaleString()}
								</p>
								<h5>Location</h5>
								{#if dive.diveSite}
									<p>
										<a href={`/sites/${dive.diveSite.slug}`}>
											{dive.diveSite.name}
										</a>
									</p>
								{/if}
							</div>
							<div class="column col-6 col-sm-12">
								<h5>Depth</h5>
								<p>
									{dive.depth.toFixed(2)}m
								</p>
								<h5>Duration</h5>
								<p>
									{formatMinutes(dive.duration)}
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>
			<div class="column col-12 col-sm-12">
				<h1 class="page-title">
					<PhotoIcon size="66px" /> Photos
				</h1>
			</div>
			<div class="column col-12 col-sm-12">
				<ImageList photos={dive.latestPhotos} query={{ dive: dive.id }} />
			</div>
		{:else}
			<div class="column col-12">
				<div class="loading loading-lg" />
			</div>
		{/if}
	</div>
</div>
