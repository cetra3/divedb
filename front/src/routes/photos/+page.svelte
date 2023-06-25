<script lang="ts">
	import ImageList from '$lib/components/ImageList.svelte';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import type { PageData } from './$types';
	export let data: PageData;

	$: photos = data.photos;
</script>

<svelte:head>
	<title>DiveDB - Photos</title>
</svelte:head>

<div class="dive-sites container grid-lg">
	<div class="columns">
		<div class="column col-6 col-lg-12">
			<h1 class="page-title">
				<PhotoIcon size="22px" /> Photos
				<a href="/photos/new">
					<button class="btn btn-secondary btn-sm">Add New</button>
				</a>
			</h1>
		</div>
	</div>
	<ImageList {photos} query={{}} />
</div>
