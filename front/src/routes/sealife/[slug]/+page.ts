import { client } from '$lib/graphql/client';
import type { PageLoad } from './$types';
import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkRehype from 'remark-rehype';
import rehypeSanitize from 'rehype-sanitize';
import rehypeStringify from 'rehype-stringify';

export const load: PageLoad = async ({ params }) => {
	let response = await client.getSealife({ slug: params.slug });
	let { categories } = await client.getCategories();

	let sealife = response?.sealife[0];
	let siteUrl = response.siteUrl;

	const mdProc = unified()
		.use(remarkParse)
		.use(remarkRehype)
		.use(rehypeSanitize)
		.use(rehypeStringify);

	const mdDesc = await mdProc.process(sealife.description);

	return {
		sealife,
		mdDesc,
		siteUrl,
		categories
	};
};
