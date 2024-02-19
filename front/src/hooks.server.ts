import type { Handle } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

export const handle: Handle = ({ event, resolve }) => {
	return resolve(event, {
		transformPageChunk: ({ html }) => html.replace('%divedb.header%', env['DIVEDB_HEADER'] ?? "")
	});
};