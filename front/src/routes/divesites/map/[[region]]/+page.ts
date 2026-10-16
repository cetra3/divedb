import { getClient } from '#lib/graphql/client.js';

import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	let response = await getClient.getRegions();

	return {
		// Not awaited, so navigating to the map doesn't wait on every dive site
		diveSites: getClient.getDiveSitesSummaryMetrics().then((response) => response.diveSites),
		regions: response.regions,
		region: params.region
	};
};
