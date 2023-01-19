<script lang="ts">
	import type { Category } from '$lib/graphql/generated';
	type CategoryMap = { [category: string]: string[] };

	export let categories: Category[];
	export let map: CategoryMap;

	$: values = categories.map((val) => val.values).flat();
</script>

{#each categories as category, catIdx}
	{#if map[category.id] !== undefined}
		<strong>{category.name}: </strong>
		{map[category.id]
			.map((catId) => values.find((val) => val.id == catId)?.value)
			.join(', ')}{#if catIdx !== categories.length - 1},&nbsp;{/if}
	{/if}
{/each}
