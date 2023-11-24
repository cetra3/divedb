import type { PageLoad } from "./$types";
import { client } from '$lib/graphql/client';

export const load: PageLoad = async ({ params }) => {
    let {user, siteUrl }= await client.getUser({ username: params.username });

    return {
        user,
        siteUrl
	};

}