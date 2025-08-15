<script lang="ts">
	import ImageList from '$lib/components/ImageList.svelte';
	import DiveSummary from '$lib/components/dives/DiveSummary.svelte';
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import MastodonIcon from '$lib/icons/MastodonIcon.svelte';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
	$: user = data.user;
	$: siteUrl = data.siteUrl;
	$: apubId = getApubId(user.username, data.siteUrl);

	const getApubId = (username: string, url: string) => {
		const domain = new URL(url).hostname;

		return `@${username}@${domain}`;
	};
</script>

<svelte:head>
	<title>DiveDB - {user.displayName ?? user.username}</title>
	<meta property="og:type" content="website" />
	<meta property="og:title" content={user.displayName ?? user.username} />
	{#if user.photoId}
		<meta property="og:image" content="{siteUrl}/api/photos/jpeg/{user.photoId}" />
	{:else}
		<meta property="og:image" content="{siteUrl}/logo.png" />
	{/if}
	<meta name="description" property="og:description" content={user.description} />
	<meta property="og:url" content="{siteUrl}/users/{user.username}" />
	<meta property="og:site_name" content="DiveDB" />
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<div class="card">
				<div class="card-image">
					<div class="columns col-gapless">
						{#if user.photoId}
							<div class="column col-4 col-sm-12">
								<img
									alt="user"
									class="profile-image img-responsive"
									src={`/api/photos/jpeg/${user.photoId}`}
								/>
							</div>
						{/if}
						<div class="column col-8 col-sm-12">
							<div class="profile-description">
								<header class="card-header">
									<h1 class="page-title">
										{user.displayName ?? user.username}
									</h1>
									<div class="title-center">
										<PhotoIcon size="22px" />
										<span class="site-metrics"> {user.photoCount} Photos </span>
									</div>
									<div class="title-center">
										<DiveLogIcon size="22px" />
										<span class="site-metrics"> {user.diveCount} Dives Logged </span>
									</div>
									<div
										class="title-center"
										title="Search for '{apubId}' on your ActivityPub instance to follow {user.displayName ??
											user.username}"
									>
										<MastodonIcon size="22px" />
										<span class="site-metrics">{apubId}</span>
									</div>
								</header>
								{#if user.description != ''}
									<section class="card-body">
										<h5>Description</h5>
										<p>
											{user.description}
										</p>
									</section>
								{/if}
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
		{#if user.latestDives.length > 0}
			<div class="column col-12 col-sm-12">
				<h1 class="page-title">
					<DiveLogIcon size="1.4em" />
					Recent Dives by {user.displayName ?? user.username}
					<a href="/users/{user.username}/dives">
						<button class="btn btn-primary btn-sm">
							View {user.displayName ?? user.username}'s Dives
						</button>
					</a>
				</h1>
			</div>
			{#each user.latestDives as dive}
				<DiveSummary {dive} />
			{/each}
		{/if}

		{#if user.latestPhotos.length > 0}
			<div class="column col-12 col-sm-12">
				<h1 class="page-title">
					<PhotoIcon size="1.4em" />
					Recent Photos by {user.displayName ?? user.username}
					<a href="/users/{user.username}/photos">
						<button class="btn btn-primary btn-sm">
							View {user.displayName ?? user.username}'s Photos
						</button>
					</a>
				</h1>
			</div>
		{/if}
	</div>
	<ImageList photos={user.latestPhotos} query={{ username: user.username }} />
</div>

<style lang="scss">
	.profile-image {
		border-radius: 0 !important;
		height: 100%;
		object-fit: cover;
	}
</style>
