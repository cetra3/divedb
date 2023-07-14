<script lang="ts">
	import PhotoLabels from '$lib/components/photos/PhotoLabels.svelte';

	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import imageAlt from '$lib/util/imageAlt';
	import ImageList from '$lib/components/ImageList.svelte';
	import Photo from '$lib/components/photos/Photo.svelte';

	import type { PageData } from './$types';
	export let data: PageData;

	let photo = data.photo;
	let relatedPhotos = data.relatedPhotos;
	let query = data.query;
	let relatedTitle = data.relatedTitle;
	let siteUrl = data.siteUrl;

	$: title = photo ? imageAlt(photo) : ' Photo';
	$: titleWithDate = photo ? imageAlt(photo, true) : ' Photo';
</script>

<svelte:head>
	<title>DiveDB - {title}</title>
	<meta property="og:type" content="website" />
	<meta property="og:title" content={title} />
	{#if photo}
		<meta name="description" property="og:description" content={titleWithDate} />
		<meta property="og:image" content="{siteUrl}/api/photos/jpeg/{photo.id}" />
		<meta property="og:url" content="{siteUrl}/photos/{photo.id}" />
	{/if}
	<meta property="og:site_name" content="DiveDB" />
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		{#if photo !== undefined}
			<div class="column col-12 col-sm-12">
				<div class="card">
					<div class="card-image">
						<Photo width={photo.width} height={photo.height} alt={title} id={photo.id} />
					</div>
					{#if titleWithDate != ''}
						<div class="card-header">
							{titleWithDate}
						</div>
					{/if}
					<div class="card-footer photo-labels no-padding">
						<PhotoLabels showEdit {photo} />
					</div>
				</div>
				<div class="column col-12 col-sm-12">
					<h1 class="page-title">
						<PhotoIcon size="33px" /> Other photos {relatedTitle}
					</h1>
				</div>
				<div class="column col-12 col-sm-12" />
			</div>
			<div class="column col-12 col-sm-12">
				{#if relatedPhotos}
					<ImageList photos={relatedPhotos} {query} />
				{/if}
			</div>
		{:else}
			<div class="column col-12">
				<div class="loading loading-lg" />
			</div>
		{/if}
	</div>
</div>
