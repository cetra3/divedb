import { client } from '$lib/graphql/client';

export const prerender = false;

export async function load() {
	try {
		let { regions } = await client.getRegions();
		let { feedback } = await client.getFeedback();

		return {
			regions,
			feedback
		};
	} catch (error) {
		return {};
	}
}
