import { client } from '$lib/graphql/client';
export const prerender = true;

import type { PageLoad } from './$types';

export const load: PageLoad = (async ({ params }) => {
	let dives = await client.getDives({ username: params.username });

	return {
		dives: dives.dives,
		username: params.username
	};
})
