<script lang="ts">
	import { goto } from '$app/navigation';
	import EditSealife from '$lib/components/forms/EditSealife.svelte';
	import SealifeIcon from '$lib/icons/SealifeIcon.svelte';
	import type { CreateSealife } from '$lib/graphql/generated';
	import { client } from '$lib/graphql/client';
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	import type { PageData } from './$types';

	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();

	let sealife: CreateSealife = {
		name: '',
		scientificName: '',
		description: '',
		hideLocation: false
	};

	let onSave = (sealife: CreateSealife) => {
		client.addSealife({ sealife }).then((val) => {
			goto(`/sealife/${val.newSealife.slug}`);
		});
	};
</script>

<CheckLogin />

<div class="container grid-lg">
	<div class="columns">
		<div class="column">
			<h1 class="page-title">
				<SealifeIcon size="33px" /> New Sealife
			</h1>
		</div>
	</div>
	<EditSealife categories={data.categories} {onSave} {sealife} />
</div>
