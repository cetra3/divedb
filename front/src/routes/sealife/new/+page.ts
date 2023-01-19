import { getClient } from '$lib/graphql/client';

export async function load() {
	let { categories } = await getClient.getCategories();

	return {
		categories
	};
}
