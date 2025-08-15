import { getClient } from '$lib/graphql/client';
import type { PageLoad } from './$types';

export const load = (async ({ url }) => {
	let photos = await getClient.getPhotos();

	return {
		photos: photos.photos
	};
}) satisfies PageLoad;
