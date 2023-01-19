import { client } from '$lib/graphql/client';
import type { PageLoad } from './$types';

export const prerender = false;

export const load: PageLoad = async ({ params }) => {
	try {
		let response = await client.getDiveSites({ id: params.id });
		let diveSite = response?.diveSites[0];

		return {
			diveSite
		};
	} catch (error) {
		return {};
	}
};
