<script lang="ts">
	import type { UserSummaryFragment } from '$lib/graphql/generated';

	export let user: UserSummaryFragment;
</script>

<span class="label label-success" title={'@' + user.username}>
	{#if user.displayName != undefined}
		{user.displayName}
	{:else}
		@{user.username}
	{/if}
</span>
