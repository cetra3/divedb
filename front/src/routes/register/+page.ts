import { client } from '$lib/graphql/client';

export async function load() {
	let result = await client.fbAppId();

	return {
		fbAppId: result.fbAppId
	};
}
