import type { PhotoSummaryFragment } from '$lib/graphql/generated';

export default function imageAlt(image: PhotoSummaryFragment, includeDate = false) {
	let imageAlt = '';
	const diveSite = image.diveSite;

	if (image.sealife) {
		imageAlt = `Photo of ${image.sealife.name}`;
		if (diveSite) {
			imageAlt += ` at ${diveSite.name}. `;
		} else {
			imageAlt += `. `;
		}
	} else if (diveSite) {
		imageAlt += `Photo at ${diveSite.name}. `;
	}

	if (includeDate && image.date) {
		imageAlt += `Taken on ${new Date(image.date).toLocaleString()}`;
	}

	return imageAlt;
}
