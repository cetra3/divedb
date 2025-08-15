import { getClient } from '$lib/graphql/client';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	let photos = await getClient.getPhotos({ username: params.username });

	return {
		photos: photos.photos,
		username: params.username
	};
};
