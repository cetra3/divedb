<script lang="ts">
	import ChatFilled from '$lib/icons/ChatFilled.svelte';
	import ChatOutline from '$lib/icons/ChatOutline.svelte';

	interface Props {
		diveId: string;
		numComments: number;
	}

	let { diveId, numComments }: Props = $props();
</script>

<a href="/dives/{diveId}#comments">
	<span class="label comment-label" class:label-warning={numComments > 0}>
		{#if numComments == 0}
			<ChatOutline size="16px" />
		{:else}
			<ChatFilled size="16px" />
		{/if}
		{numComments > 0 ? numComments : ''}
	</span></a
>

<style lang="scss" global>
	.comment-label {
		background-color: #fff5db;
		svg {
			vertical-align: -0.05rem;
		}
	}
</style>
