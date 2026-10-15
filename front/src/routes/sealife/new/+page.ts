import { getClient } from '#lib/graphql/client.js';

export async function load() {
	let { categories } = await getClient.getCategories();

	return {
		categories
	};
}
