import { client } from '$lib/graphql/client';
import type { PageLoad } from './$types';

export const prerender = false;

export const load: PageLoad = async ({ params }) => {
	try {
		let response = await client.getDive({ id: params.id });

		let dive = response.dives[0];

		return {
			dive
		};
	} catch (error) {
		return {};
	}
};
