<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import HeartFilled from '$lib/icons/HeartFilled.svelte';
	import HeartOutline from '$lib/icons/HeartOutline.svelte';
	import { session } from '$lib/session';

	export let liked: boolean;
	export let likes: number;

	const dispatch = createEventDispatcher<{ liked: boolean }>();

	const onClick = (e: MouseEvent) => {
		e.preventDefault();

		if ($session.loggedIn) {
			if (liked) {
				likes = likes - 1;
				liked = false;
			} else {
				likes = likes + 1;
				liked = true;
			}
			dispatch('liked', liked);
		}
	};
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
{#if $session.loggedIn || likes > 0}
	<span
		class="label heart-label"
		class:pointer={$session.loggedIn}
		class:liked={liked || !$session.loggedIn}
		class:un-liked={!liked && $session.loggedIn}
		on:click={onClick}
	>
		{#if liked}
			<HeartFilled size="16px" />
		{:else}
			<HeartOutline size="16px" />
		{/if}
		{likes > 0 ? likes : ''}
	</span>
{/if}

<style lang="scss" global>
	.heart-label {
		svg {
			vertical-align: -0.1rem;
		}
	}

	.liked {
		background-color: #d67ca5;
		color: white;
	}

	.un-liked {
		background-color: #fceff4;
	}
</style>
