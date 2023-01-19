import { client } from '$lib/graphql/client';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	let photos = await client.getPhotos({ id: params.id });

	return {
		photo: photos.photos[0]
	};
};
