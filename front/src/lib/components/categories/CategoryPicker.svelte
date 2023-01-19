<script lang="ts">
	import type { Category } from '$lib/graphql/generated';
	type CategoryMap = { [category: string]: string[] };

	export let categories: Category[];
	export let map: CategoryMap;

	const addValue = (categoryId: string, categoryValueId: string) => {
		let catVals = map[categoryId] ?? [];
		catVals.push(categoryValueId);
		map[categoryId] = catVals;
	};

	const removeValue = (categoryId: string, categoryValueId: string) => {
		let catVals = map[categoryId] ?? [];
		map[categoryId] = catVals.filter((val) => val !== categoryValueId);
	};

	$: values = Object.values(map).flat();
</script>

{#each categories as category}
	<div class="category-row">
		<div class="category-name">{category.name}</div>
		<div class="category-buttons">
			{#each category.values as value}
				<button
					on:click={(e) => {
						e.preventDefault();
						if (values.some((val) => val == value.id)) {
							removeValue(category.id, value.id);
						} else {
							addValue(category.id, value.id);
						}
					}}
					class="btn btn-sm"
					class:btn-primary={values.some((val) => val == value.id)}
				>
					{#if category.name === 'Colour'}
						<span class="colour-selection" style={'background-color:' + value.value} />
					{/if}
					{value.value}
				</button>
			{/each}
		</div>
	</div>
{/each}

<style lang="scss">
	.btn-primary {
		.colour-selection {
			border: 1px solid white;
		}
	}
	.colour-selection {
		border: 0.05rem solid #0e5e7c;
		border-radius: 3px;
		display: inline-block;
		width: 10px;
		height: 10px;
		background-color: black;
	}
	.category-row {
		display: inline-block;
		margin-bottom: 0.3rem;
		margin-right: 0.5rem;
	}
	.category-buttons {
		display: inline-block;
		.btn {
			margin: 0.2rem 0.1rem;
		}
	}
	.category-name {
		display: inline-block;
	}
</style>
