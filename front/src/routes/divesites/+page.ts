import { getClient } from '$lib/graphql/client';

export async function load() {
	let diveSites = await getClient.getDiveSitesSummaryMetrics();

	return {
		diveSites
	};
}
