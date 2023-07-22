import { getClient } from '$lib/graphql/client';
import type { PageLoad } from './$types';

export const load = (async ({ url }) => {
	let username = url.searchParams.get('u') || undefined;

	let photos = await getClient.getPhotos({ username });

	return {
		photos: photos.photos,
		username
	};
}) satisfies PageLoad;
