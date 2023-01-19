<script lang="ts">
	import { onMount } from 'svelte';
	import { client } from '$lib/graphql/client';
	import { goto } from '$app/navigation';
	import { session } from '$lib/session';
	import { fbRegisterRedirect } from '$lib/util/fbRedirect';
	import type { ClientError } from 'graphql-request';

	let errors: string | undefined = undefined;

	let loading = true;

	onMount(() => {
		let searchParams = new URLSearchParams(window.location.search);

		let code = searchParams.get('code') ?? '';
		let redirect = searchParams.get('redirect') || '/';

		client
			.fbRegisterUser({ redirectUri: fbRegisterRedirect, code })
			.then((val) => {
				localStorage.setItem('token', val.fbRegisterUser.token);
				session.set({ loggedIn: true, user: val.fbRegisterUser });
				loading = false;
				goto(redirect);
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	});
</script>

<div class="container grid-lg">
	<div class="columns">
		{#if errors}
			<div class="toast">{errors}</div>
		{/if}
		{#if loading}
			<div class="column col-12">
				<div class="loading loading-lg" />
			</div>
		{/if}
	</div>
</div>
