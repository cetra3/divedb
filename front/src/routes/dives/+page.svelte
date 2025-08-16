<script lang="ts">
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import type { PageData } from './$types';
	import { session } from '$lib/session';
	import DiveList from '$lib/components/dives/DiveList.svelte';

	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();
	let dives = $derived(data.dives);
</script>

<svelte:head>
	<title>DiveDB - Dives</title>
</svelte:head>
<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-lg-12">
			<h1 class="page-title">
				<DiveLogIcon size="1.4em" />
				Dives

				{#if $session.loggedIn}
					<a href="/users/{$session.user?.username}/dives">
						<button class="btn btn-secondary btn-sm">Your Dives</button>
					</a>

					<a href="/dives/new">
						<button class="btn btn-secondary btn-sm">Add New</button>
					</a>
					<a href="/dives/subsurface">
						<button class="btn btn-secondary btn-sm">Sync from Subsurface</button>
					</a>
				{/if}
			</h1>
		</div>
	</div>
	{#if dives !== undefined}
		<DiveList {dives} />
	{:else}
		<div class="column col-12">
			<div class="loading loading-lg"></div>
		</div>
	{/if}
</div>
