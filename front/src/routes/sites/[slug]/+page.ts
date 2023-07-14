import { client } from '$lib/graphql/client';
import type { PageLoad } from './$types';
import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkRehype from 'remark-rehype';
import rehypeSanitize from 'rehype-sanitize';
import rehypeStringify from 'rehype-stringify';

export const load: PageLoad = async ({ params }) => {
	try {
		let response = await client.getDiveSites({ slug: params.slug });
		let diveSite = response?.diveSites[0];
		let siteUrl = response?.siteUrl;

		const mdProc = unified()
			.use(remarkParse)
			.use(remarkRehype)
			.use(rehypeSanitize)
			.use(rehypeStringify);

		const mdDesc = await mdProc.process(diveSite.description);
		const mdAccess = await mdProc.process(diveSite.access);

		return {
			diveSite,
			siteUrl,
			mdDesc,
			mdAccess
		};
	} catch (error) {
		return {};
	}
};
