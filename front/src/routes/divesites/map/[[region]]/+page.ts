import { getClient } from '$lib/graphql/client';

import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	let response = await getClient.getDiveSites();
	let diveSites = response?.diveSites;
	let regions = response?.regions;

	return {
		diveSites,
		regions,
		region: params.region
	};
};
