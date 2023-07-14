<script lang="ts">
	import type { PhotoSummaryFragment } from '$lib/graphql/generated';
	export let showEdit = false;
	import { session } from '$lib/session';
	import { client } from '$lib/graphql/client';
	import LikeHeart from '../forms/LikeHeart.svelte';
	import UserLabel from '../labels/UserLabel.svelte';

	export let photo: PhotoSummaryFragment;

	const onLike = (liked: CustomEvent<boolean>) => {
		let photoId = photo.id;

		if (liked.detail) {
			client.likePhoto({ photoId });
		} else {
			client.unlikePhoto({ photoId });
		}
	};
</script>

<div class="label-list">
	<LikeHeart liked={photo.liked} likes={photo.likes} on:liked={onLike} />

	<UserLabel user={photo.user} />

	{#if photo.sealife}
		<a href={`/sealife/${photo.sealife.slug}`}>
			<span class="label label-error">
				{photo.sealife.name}
			</span>
		</a>
	{/if}
	{#if photo.diveSite}
		<a href={`/sites/${photo.diveSite.slug}`}>
			<span class="label label-primary">{photo.diveSite.name}</span>
		</a>
	{/if}
	{#if photo.date}
		{#if photo.dive && $session.user?.id === photo.userId}
			<a href={`/dives/${photo.dive.id}`}>
				<span class="label">{new Date(photo.date).toLocaleDateString()}</span>
			</a>
		{:else}
			<span class="label">{new Date(photo.date).toLocaleDateString()}</span>
		{/if}
	{/if}
	{#if showEdit}
		<a href={`/photos/edit/${photo.id}`}>
			<span class="label label-secondary"> Edit</span>
		</a>
	{:else}
		<a href={`/photos/${photo.id}`}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="22"
				height="22"
				fill="currentColor"
				class="photo-link"
				viewBox="0 0 16 16"
			>
				<path
					d="M4.715 6.542 3.343 7.914a3 3 0 1 0 4.243 4.243l1.828-1.829A3 3 0 0 0 8.586 5.5L8 6.086a1.002 1.002 0 0 0-.154.199 2 2 0 0 1 .861 3.337L6.88 11.45a2 2 0 1 1-2.83-2.83l.793-.792a4.018 4.018 0 0 1-.128-1.287z"
				/>
				<path
					d="M6.586 4.672A3 3 0 0 0 7.414 9.5l.775-.776a2 2 0 0 1-.896-3.346L9.12 3.55a2 2 0 1 1 2.83 2.83l-.793.792c.112.42.155.855.128 1.287l1.372-1.372a3 3 0 1 0-4.243-4.243L6.586 4.672z"
				/>
			</svg>
		</a>
	{/if}

	<slot />
</div>

<style global lang="scss">
	.label-list {
		.label {
			margin-bottom: 0.4rem;
		}

		padding-bottom: 0.4rem;
	}
</style>
