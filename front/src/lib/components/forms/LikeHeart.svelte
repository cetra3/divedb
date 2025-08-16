<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import HeartFilled from '$lib/icons/HeartFilled.svelte';
	import HeartOutline from '$lib/icons/HeartOutline.svelte';
	import { session } from '$lib/session';

	interface Props {
		liked: boolean;
		likes: number;
	}

	let { liked = $bindable(), likes = $bindable() }: Props = $props();

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

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
{#if $session.loggedIn || likes > 0}
	<span
		class="label heart-label"
		class:pointer={$session.loggedIn}
		class:liked={liked || !$session.loggedIn}
		class:un-liked={!liked && $session.loggedIn}
		onclick={onClick}
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
