import { client } from '$lib/graphql/client';

export async function load() {
	let { regions } = await client.getRegions();

	return {
		regions
	};
}
