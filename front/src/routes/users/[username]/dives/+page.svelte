<script lang="ts">
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import type { PageData } from './$types';
	import { session } from '$lib/session';
	import DiveList from '$lib/components/dives/DiveList.svelte';

	export let data: PageData;
	$: dives = data.dives;
	$: username = data.username;
</script>

<svelte:head>
	<title>DiveDB - Dives by @{username}</title>
</svelte:head>
<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-lg-12">
			<h1 class="page-title">
				<DiveLogIcon size="1.4em" />
				Dives by @{username}

				<a href="/dives">
					<button class="btn btn-secondary btn-sm">All Dives</button>
				</a>
				<a href="/users/{username}">
					<button class="btn btn-secondary btn-sm">View Profile</button>
				</a>

				<a href="/dives/new">
					<button class="btn btn-secondary btn-sm">Add New</button>
				</a>
				<a href="/dives/subsurface">
					<button class="btn btn-secondary btn-sm">Sync from Subsurface</button>
				</a>
			</h1>
		</div>
	</div>
	{#if dives !== undefined}
		<DiveList {dives} query={{ username }} />
	{:else}
		<div class="column col-12">
			<div class="loading loading-lg" />
		</div>
	{/if}
</div>
