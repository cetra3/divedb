import { client } from '$lib/graphql/client';
import type { DiveWithNumberFragment } from '$lib/graphql/generated';
import type { PageLoad } from './$types';

export const prerender = false;

export const load: PageLoad = async ({ params }) => {
	try {
		let response = await client.getDive({ id: params.id });

		let dive = response.dives[0];

		let relatedDives: DiveWithNumberFragment[] = [];

		if (dive && dive.diveSiteId) {
			let relatedResponse = await client.getDivesWithNumber({ diveSite: dive.diveSiteId });

			relatedDives = relatedResponse.dives.filter((val) => val.id != dive.id);
		}

		return {
			dive,
			relatedDives
		};
	} catch (error) {
		return {};
	}
};
