import { client } from '#lib/graphql/client.js';

import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
	let dives = await client.getDives();

	return {
		dives: dives.dives
	};
};
