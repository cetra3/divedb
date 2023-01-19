import { client } from '$lib/graphql/client';
import type { PageLoad } from './$types';

export const prerender = false;

export const load: PageLoad = async ({ params }) => {
	let response = await client.getRegions();

	let region = response.regions.find((val) => val.id == params.id);

	return {
		region
	};
};
