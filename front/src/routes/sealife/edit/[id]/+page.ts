import { client } from '$lib/graphql/client';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	try {
		let response = await client.getSealife({ id: params.id });
		let sealife = response?.sealife[0];
		let { categories } = await client.getCategories();

		return {
			sealife,
			categories
		};
	} catch (error) {
		return {
			categories: []
		};
	}
};
