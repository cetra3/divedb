<script lang="ts">
	import ImageList from '$lib/components/ImageList.svelte';
	import type { CreateDiveSite, SiteFragment } from '$lib/graphql/generated';
	import SiteMetrics from '$lib/components/SiteMetrics.svelte';
	import DiveSiteIcon from '$lib/icons/DiveSiteIcon.svelte';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import Photo from '$lib/components/photos/Photo.svelte';
	import { client } from '$lib/graphql/client';
	import { session } from '$lib/session';

	import type { PageData } from './$types';
	import References from '$lib/components/References.svelte';
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import DiveList from '$lib/components/dives/DiveList.svelte';
	export let data: PageData;

	$: diveSite = data.diveSite;
	$: siteUrl = data.siteUrl;
	$: mdDesc = data.mdDesc;
	$: mdAccess = data.mdAccess;

	$: photos = diveSite?.latestPhotos ?? [];

	$: dives = diveSite?.latestDives ?? [];

	let loading = false;
	let changed = false;

	const toDegreeDecimalMinutes = (lat: number, lon: number) => {
		let lat_d = Math.floor(Math.abs(lat));
		let lat_dm = (Math.abs(lat) % 1) * 60;
		let lat_dir = lat > 0 ? 'N' : 'S';

		let lon_d = Math.floor(Math.abs(lon));
		let lon_dm = (Math.abs(lon) % 1) * 60;
		let lon_dir = lon > 0 ? 'E' : 'W';

		return `${lat_d}°${lat_dm.toFixed(3)}'${lat_dir} ${lon_d}°${lon_dm.toFixed(3)}'${lon_dir}`;
	};

	const changeCoverPhoto = (photoId: string) => {
		if (diveSite) {
			let site: CreateDiveSite = {
				id: diveSite.id,
				name: diveSite.name,
				description: diveSite.description,
				access: diveSite.access,
				depth: diveSite.depth,
				difficulty: diveSite.difficulty,
				published: diveSite.published,
				lat: diveSite.lat,
				lon: diveSite.lon,
				photoId: photoId
			};

			loading = true;
			client
				.addDiveSite({
					site
				})
				.then((val) => {
					diveSite = val.newDiveSite;
					loading = false;
					changed = true;
				});
		}
	};
</script>

<svelte:head>
	<title>DiveDB - {diveSite?.name}</title>
	<meta property="og:type" content="website" />
	<meta property="og:title" content={diveSite?.name} />
	{#if diveSite?.photoId}
		<meta property="og:image" content="{siteUrl}/api/photos/jpeg/{diveSite.photoId}" />
	{:else}
		<meta property="og:image" content="{siteUrl}/logo.png" />
	{/if}
	<meta name="description" property="og:description" content={diveSite?.summary} />
	<meta property="og:url" content="{siteUrl}/sites/{diveSite?.slug}" />
	<meta property="og:site_name" content="DiveDB" />
</svelte:head>

{#if !diveSite}
	<div class="container grid-lg">
		<div class="columns">
			<div class="toast">
				This site could not be found. Please check if you're logged in or that the ID exists
			</div>
		</div>
	</div>
{:else}
	<div class="container grid-lg">
		<div class="columns">
			<div class="column col-12 col-sm-12">
				<article itemscope itemID={diveSite.id} itemType="https://schema.org/Thing" class="card">
					<div class="card-image">
						{#if diveSite.photo && diveSite.photoId}
							<Photo
								lazy={false}
								width={diveSite.photo.width}
								height={diveSite.photo.height}
								id={diveSite.photoId}
								alt={diveSite.name}
							/>
						{:else}
							<div class="hero bg-dark flex-center">
								<div class="hero-body">
									<DiveSiteIcon size="66px" />
								</div>
							</div>
						{/if}
					</div>
					<header class="card-header">
						<h1 class="page-title">
							<DiveSiteIcon size="22px" />
							<span itemprop="name">{diveSite.name}</span>
							<a href={`/divesites/edit/${diveSite.id}`}>
								<button class="btn btn-secondary btn-sm"> Edit </button>
							</a>
							{#if diveSite.lat != 0}
								<a href={`/divesites/map?site=${diveSite.slug}`}>
									<button class="btn btn-secondary btn-sm"> View on Map </button>
								</a>
							{/if}
						</h1>
						<div class="card-subtitle title-center">
							<SiteMetrics site={diveSite} />
						</div>
						<strong>Depth: </strong>
						{diveSite.depth}m,
						<strong>Difficulty: </strong>{diveSite.difficulty}

						{#if diveSite.lat != 0}
							<br />
							<strong> GPS:</strong>
							<span itemProp="geo" itemScope itemType="https://schema.org/GeoCoordinates">
								{diveSite.lat}
								{diveSite.lon}
								<meta itemProp="latitude" content={diveSite.lat + ''} />
								<meta itemProp="longitude" content={diveSite.lon + ''} />
								<span class="hide-small">
									( {toDegreeDecimalMinutes(diveSite.lat, diveSite.lon)} )
								</span>
							</span>
						{/if}
						<br />
						<strong> Last Edited: </strong>
						{new Date(diveSite.date).toLocaleString()}
					</header>
					<section class="card-body">
						<h2>Description</h2>
						<div itemprop="description">
							{@html mdDesc}
						</div>
						<h2>Access</h2>
						{@html mdAccess}
						<References
							showEdit={$session.loggedIn}
							diveSiteId={diveSite.id}
							references={diveSite.references}
						/>
					</section>
				</article>
			</div>
			{#if dives.length > 0}
				<div class="column col-12 col-sm-12">
					<h1 class="page-title">
						<DiveLogIcon size="33px" /> Dives
					</h1>
				</div>
				<div class="column col-12 col-sm-12">
					<DiveList {dives} query={{ diveSite: diveSite.id }} />
				</div>
			{/if}
			<div class="column col-12 col-sm-12">
				<h1 class="page-title">
					<PhotoIcon size="33px" /> Photos
				</h1>
			</div>
			<div class="column col-12 col-sm-12">
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<ImageList {photos} query={{ diveSite: diveSite.id }}>
					<svelte:fragment let:photo slot="photo-label">
						{#if diveSite.photoId !== photo}
							{#if loading}
								<span class="loading padding-20"></span>
							{:else}
								<span
									class="label pointer"
									on:click={() => {
										changeCoverPhoto(photo);
									}}>Make Cover Photo</span
								>
							{/if}
						{:else if changed}
							<i class="icon icon-check"></i>
						{/if}
					</svelte:fragment>
				</ImageList>
			</div>
		</div>
	</div>
{/if}
