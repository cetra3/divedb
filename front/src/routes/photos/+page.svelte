<script lang="ts">
	import ImageList from '$lib/components/ImageList.svelte';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import type { PageData } from './$types';
	import { session } from '$lib/session';
	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();

	let photos = $derived(data.photos);
</script>

<svelte:head>
		<title>DiveDB - Photos</title>
</svelte:head>

<div class="dive-sites container grid-lg">
	<div class="columns">
		<div class="column col-12 col-lg-12">
			<h1 class="page-title">
				<PhotoIcon size="22px" /> Photos
				{#if $session.loggedIn}
					<a href="/users/{$session.user?.username}/photos">
						<button class="btn btn-secondary btn-sm">Your Photos</button>
					</a>
				{/if}
				<a href="/photos/new">
					<button class="btn btn-secondary btn-sm">Add New</button>
				</a>
			</h1>
		</div>
	</div>
	<ImageList {photos} />
</div>
