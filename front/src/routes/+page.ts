import { getClient } from '$lib/graphql/client';

export async function load() {
	let frontPage = await getClient.frontPage();

	return {
		frontPage
	};
}
