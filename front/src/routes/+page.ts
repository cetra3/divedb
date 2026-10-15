import { getClient } from '#lib/graphql/client.js';

export async function load() {
	let frontPage = await getClient.frontPage();

	return {
		frontPage
	};
}
