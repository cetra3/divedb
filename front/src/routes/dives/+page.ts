import { client } from '$lib/graphql/client';

export async function load() {
	try {
		let dives = await client.getDives();

		return {
			dives: dives.dives
		};
	} catch (error) {
		return {};
	}
}
