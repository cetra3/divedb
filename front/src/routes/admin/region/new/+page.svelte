<script lang="ts">
	import { goto } from '$app/navigation';
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	import FormRow from '$lib/components/FormRow.svelte';
	import EditRegion from '$lib/components/forms/EditRegion.svelte';
	import { client } from '$lib/graphql/client';
	import type { CreateRegion } from '$lib/graphql/generated';
	import type { ClientError } from 'graphql-request';

	let errors: string | undefined = undefined;

	const onSave = (region: CreateRegion) => {
		loading = true;
		client
			.newRegion({ region })
			.then(() => {
				loading = false;
				goto('/admin');
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	let loading = false;
</script>

<CheckLogin />

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12">
			<h1 class="page-title">Add Region</h1>
		</div>
		<div class="column col-12">
			<EditRegion {onSave} />
		</div>
		{#if errors}
			<div class="column col-12">
				<div class="toast">{errors}</div>
			</div>
		{/if}
		{#if loading}
			<div class="column col-12">
				<div class="loading loading-lg" />
			</div>
		{/if}
	</div>
</div>
