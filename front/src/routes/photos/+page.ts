import { getClient } from '$lib/graphql/client';

export async function load() {
	let photos = await getClient.getPhotos();

	return {
		photos: photos.photos
	};
}
