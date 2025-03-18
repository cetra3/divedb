<script lang="ts">
	import ImageList from '$lib/components/ImageList.svelte';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import type { PageData } from './$types';
	export let data: PageData;
	import { session } from '$lib/session';

	$: photos = data.photos;
	$: username = data.username;
</script>

<svelte:head>
	{#if username != undefined}
		<title>DiveDB - Photos by @{username}</title>
	{:else}
		<title>DiveDB - Photos</title>
	{/if}
</svelte:head>

<div class="dive-sites container grid-lg">
	<div class="columns">
		<div class="column col-12 col-lg-12">
			<h1 class="page-title">
				<PhotoIcon size="22px" /> Photos {#if username != undefined}by @{username}{/if}
				{#if username != undefined}
					<a href="/photos">
						<button class="btn btn-secondary btn-sm">All Photos</button>
					</a>
				{:else if $session.loggedIn}
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
	<ImageList {photos} query={{ username }} />
</div>
