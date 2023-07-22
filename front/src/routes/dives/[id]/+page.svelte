<script lang="ts">
	import DiveGraph from '$lib/components/dives/DiveGraph.svelte';

	import type { PageData } from './$types';
	import { session } from '$lib/session';

	export let data: PageData;
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import ImageList from '$lib/components/ImageList.svelte';
	import formatMinutes from '$lib/util/formatMinutes';
	import DiveLabels from '$lib/components/dives/DiveLabels.svelte';
	import Comments from '$lib/components/Comments.svelte';

	$: dive = data.dive;
	$: relatedDives = data.relatedDives;
	$: mdDesc = data.mdDesc;

	$: isEditor =
		$session.user?.level == 'ADMIN' ||
		$session.user?.level == 'EDITOR' ||
		!(dive && 'userId' in dive) ||
		($session.user?.id != undefined && $session.user.id === dive.userId);
	$: title = dive
		? `${dive.user.displayName ?? '@' + dive.user.username} - #${dive.number} ${
				dive.diveSite ? ` - ${dive.diveSite.name}` : ''
		  }`
		: '';
</script>

<svelte:head>
	<title>DiveDB - {title}</title>
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		{#if dive !== undefined}
			<div class="column col-12 col-sm-12">
				<div class="card">
					<DiveGraph {dive} />
					<div class="card-header">
						<h1 class="page-title">
							<DiveLogIcon size="33px" />
							{title}
							{#if isEditor}
								<a href={`/dives/edit/${dive.id}`}>
									<button class="btn btn-secondary btn-sm"> Edit </button>
								</a>
							{/if}
						</h1>
						<strong>Date: </strong>
						{new Date(dive.date).toLocaleString()}
						<br />
						{#if dive.diveSite}
							<strong>Site: </strong>
							<a href={`/sites/${dive.diveSite.slug}`}>{dive.diveSite.name}</a>,
							<strong>Dives: </strong>
							{(relatedDives?.length ?? 0) + 1}
							<br />
						{/if}

						<strong>Max Depth: </strong>
						{dive.depth.toFixed(2)}m,
						<strong>Duration: </strong>
						{formatMinutes(dive.duration)}
						<br />
					</div>
					<div class="card-body">
						{#if dive.description != ''}
							<div class="columns">
								<div class="column col-12">
									<h2>Description</h2>
									<div itemprop="description">
										{@html mdDesc}
									</div>
								</div>
							</div>
						{/if}
					</div>
					<div class="card-footer no-padding">
						<DiveLabels {dive} />
					</div>
				</div>
			</div>

			<div class="column col-12 col-sm-12">
				<Comments {dive} />
			</div>
			{#if dive.latestPhotos.length > 0}
				<div class="column col-12 col-sm-12">
					<h1 class="page-title">
						<PhotoIcon size="66px" /> Photos
					</h1>
				</div>
				<div class="column col-12 col-sm-12">
					<ImageList photos={dive.latestPhotos} query={{ dive: dive.id }} />
				</div>
			{/if}
		{:else}
			<div class="column col-12">
				<div class="loading loading-lg" />
			</div>
		{/if}
	</div>
</div>
