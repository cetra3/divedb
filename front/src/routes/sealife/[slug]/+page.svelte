<script lang="ts">
	import type { CreateSealife } from '$lib/graphql/generated';
	import type { PageData } from './$types';
	import SealifeIcon from '$lib/icons/SealifeIcon.svelte';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import ImageList from '$lib/components/ImageList.svelte';
	import Photo from '$lib/components/photos/Photo.svelte';
	import { client } from '$lib/graphql/client';
	import CategoryView from '$lib/components/categories/CategoryView.svelte';
	import References from '$lib/components/References.svelte';
	import { session } from '$lib/session';
	export let data: PageData;
	let sealife = data.sealife;
	let mdDesc = data.mdDesc;
	let siteUrl = data.siteUrl;
	let categories = data.categories;

	let photos = sealife?.latestPhotos;

	let loading = false;
	let changed = false;

	const changeCoverPhoto = (photoId: string) => {
		if (sealife) {
			let newSealife: CreateSealife = {
				id: sealife.id,
				name: sealife.name,
				scientificName: sealife.scientificName,
				description: sealife.description,
				hideLocation: sealife.hideLocation,
				photoId
			};

			loading = true;
			client
				.addSealife({
					sealife: newSealife
				})
				.then((val) => {
					sealife = val.newSealife;
					loading = false;
					changed = true;
				});
		}
	};
</script>

<svelte:head>
	<title>DiveDB - {sealife.name}</title>
	<meta property="og:type" content="website" />
	<meta property="og:title" content={sealife.name} />
	{#if sealife.photoId}
		<meta property="og:image" content="{siteUrl}/api/photos/jpeg/{sealife.photoId}" />
	{:else}
		<meta property="og:image" content="{siteUrl}/logo.png" />
	{/if}
	<meta name="description" property="og:description" content={sealife.summary} />
	<meta property="og:url" content="{siteUrl}/sealife/{sealife.slug}" />
	<meta property="og:site_name" content="DiveDB" />
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<article itemScope itemID={sealife.id} itemType="https://schema.org/Thing" class="card">
				<div class="card-image">
					{#if sealife.photo && sealife.photo.id}
						<Photo
							lazy={false}
							width={sealife.photo.width}
							height={sealife.photo.height}
							id={sealife.photo.id}
							alt={sealife.name}
						/>
					{:else}
						<div class="hero bg-dark flex-center">
							<div class="hero-body">
								<SealifeIcon size="66px" />
							</div>
						</div>
					{/if}
				</div>

				<header class="card-header">
					<h1 class="page-title">
						<SealifeIcon size="22px" />
						<span itemProp="name">{sealife.name}</span>{' '}

						<a href={`/sealife/edit/${sealife.id}`}>
							<button class="btn btn-secondary btn-sm">Edit</button>
						</a>
					</h1>
					{#if sealife.scientificName}
						<strong>Scientific Name: </strong>
						{sealife.scientificName},
					{/if}
					<strong> Last Edited: </strong>
					{new Date(sealife.date).toLocaleString()}
					<br />
					<CategoryView {categories} map={sealife.categoryMap} />
				</header>
				<section class="card-body">
					<h2>Description</h2>
					<div itemProp="description">
						{@html mdDesc}
					</div>
					<References
						showEdit={$session.loggedIn}
						sealifeId={sealife.id}
						references={sealife.references}
					/>
				</section>
			</article>
		</div>
		<div class="column col-12 col-sm-12">
			<h1 class="page-title">
				<PhotoIcon size="33px" /> Photos
			</h1>
		</div>
		<div class="column col-12 col-sm-12">
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<ImageList {photos} query={{ sealifeId: sealife.id }}>
				<svelte:fragment let:photo slot="photo-label">
					{#if sealife.photoId !== photo}
						{#if loading}
							<span class="loading padding-20" />
						{:else}
							<span
								class="label pointer"
								on:click={() => {
									changeCoverPhoto(photo);
								}}>Make Cover Photo</span
							>
						{/if}
					{:else if changed}
						<i class="icon icon-check" />
					{/if}
				</svelte:fragment>
			</ImageList>
		</div>
	</div>
</div>
