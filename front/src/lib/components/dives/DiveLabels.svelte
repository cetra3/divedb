<script lang="ts">
	import type { DiveWithMetricsFragment, DiveNodeFragment } from '$lib/graphql/generated';

	export let dive: DiveWithMetricsFragment | DiveNodeFragment;

	import { client } from '$lib/graphql/client';
	import LikeHeart from '../forms/LikeHeart.svelte';
	import UserLabel from '../labels/UserLabel.svelte';
	import DiveComment from './DiveComment.svelte';

	const onLike = (liked: CustomEvent<boolean>) => {
		let diveId = dive.id;

		if (liked.detail) {
			client.likeDive({ diveId });
		} else {
			client.unlikeDive({ diveId });
		}
	};
</script>

<div class="label-list">
	<LikeHeart liked={dive.liked} likes={dive.likes} on:liked={onLike} />

	<DiveComment diveId={dive.id} numComments={dive.numComments} />

	<a href={`/users/${dive.user.username}`}>
		<UserLabel user={dive.user} />
	</a>
	{#if dive.diveSite}
		<a href={`/sites/${dive.diveSite.slug}`}>
			<span class="label label-primary">{dive.diveSite.name}</span>
		</a>
	{/if}
	{#if dive.date}
		<span class="label">{new Date(dive.date).toLocaleDateString()}</span>
	{/if}
</div>

<style global lang="scss">
	.label-list {
		.label {
			margin-bottom: 0.4rem;
		}

		padding-bottom: 0.4rem;
	}
</style>
