import { client } from '$lib/graphql/client';
import type { DiveWithMetricsFragment } from '$lib/graphql/generated';
import type { PageLoad } from './$types';
import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkRehype from 'remark-rehype';
import rehypeSanitize from 'rehype-sanitize';
import rehypeStringify from 'rehype-stringify';

export const load: PageLoad = async ({ params }) => {
	try {
		let response = await client.getDive({ id: params.id });

		let dive = response.dives[0];

		let relatedDives: DiveWithMetricsFragment[] = [];

		let mdDesc = undefined;

		if (dive && dive.diveSiteId) {
			const mdProc = unified()
				.use(remarkParse)
				.use(remarkRehype)
				.use(rehypeSanitize)
				.use(rehypeStringify);

			mdDesc = await mdProc.process(dive.description);

			let relatedResponse = await client.getDives({
				diveSite: dive.diveSiteId
			});

			relatedDives = relatedResponse.dives.filter((val) => val.id != dive.id);
		}

		return {
			dive,
			relatedDives,
			mdDesc,
			siteUrl: response.siteUrl
		};
	} catch (error) {
		return {};
	}
};
