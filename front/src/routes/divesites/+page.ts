import { getClient } from '#lib/graphql/client.js';

export async function load() {
	let diveSites = await getClient.getDiveSitesSummaryMetrics();

	return {
		diveSites
	};
}
