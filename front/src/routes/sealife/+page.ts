import { getClient } from '#lib/graphql/client.js';

export async function load() {
	try {
		let { sealife } = await getClient.getSealifeSummary();
		let { categories } = await getClient.getCategories();

		return {
			sealife,
			categories
		};
	} catch (error) {
		return {
			categories: []
		};
	}
}
