import { client } from '$lib/graphql/client';

export async function load() {
	let result = await client.loginInfo();

	return {
		loginInfo: result
	};
}
