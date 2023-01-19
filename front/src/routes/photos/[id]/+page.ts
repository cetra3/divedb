import { client } from '$lib/graphql/client';
import type { GetPhotosQueryVariables, PhotoSummaryFragment } from '$lib/graphql/generated';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	try {
		let response = await client.getPhotos({ id: params.id });

		let photo = response.photos[0];

		let query: GetPhotosQueryVariables | undefined = undefined;
		let relatedTitle: string | undefined = undefined;
		let relatedPhotos: PhotoSummaryFragment[] = [];
		if (photo) {
			if (photo.sealife?.id) {
				query = { sealifeId: photo.sealife?.id };
				relatedTitle = `of ${photo.sealife.name}`;
			} else if (photo.diveSite?.id) {
				query = { diveSite: photo.diveSite?.id };
				relatedTitle = `of ${photo.diveSite.name}`;
			} else {
				query = { userId: photo.userId };
				relatedTitle = `by the photographer`;
			}

			let relatedResponse = await client.getPhotos(query);

			relatedPhotos = relatedResponse.photos;
		}

		let siteUrl = response.siteUrl;

		return {
			photo,
			query,
			relatedPhotos,
			relatedTitle,
			siteUrl
		};
	} catch (error) {
		return {};
	}
};
