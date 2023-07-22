import { client } from '$lib/graphql/client';

import type { PageLoad } from './$types';

export const load = (async ({ url }) => {
	let username = url.searchParams.get('u') || undefined;
	let dives = await client.getDives({ username });

	return {
		dives: dives.dives,
		username
	};
}) satisfies PageLoad;
