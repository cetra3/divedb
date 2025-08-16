<script lang="ts">
	import { onMount } from 'svelte';
	import { client } from '$lib/graphql/client';
	import { goto } from '$app/navigation';
	import { session } from '$lib/session';
	import { fbLoginRedirect } from '$lib/util/fbRedirect';
	import type { ClientError } from 'graphql-request';

	let errors: string | undefined = $state(undefined);

	let loading = $state(true);

	onMount(() => {
		let searchParams = new URLSearchParams(window.location.search);

		let code = searchParams.get('code') ?? '';
		let redirect = searchParams.get('redirect') || '/';

		client
			.fbLoginUser({ redirectUri: fbLoginRedirect, code })
			.then((val) => {
				localStorage.setItem('token', val.fbLogin.token);
				session.set({ loggedIn: true, user: val.fbLogin });
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
				<div class="loading loading-lg"></div>
			</div>
		{/if}
	</div>
</div>
