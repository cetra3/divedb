<script lang="ts">
	import type { UserSummaryFragment } from '$lib/graphql/generated';

	interface Props {
		user: UserSummaryFragment;
	}

	let { user }: Props = $props();
</script>

<span class="label label-success" title={'@' + user.username}>
	{#if user.displayName != undefined}
		{user.displayName}
	{:else}
		@{user.username}
	{/if}
</span>
