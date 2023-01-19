<script lang="ts">
	import { goto } from '$app/navigation';
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	import EditDiveSite from '$lib/components/forms/EditDiveSite.svelte';
	import { client } from '$lib/graphql/client';
	import { Difficulty, type CreateDiveSite } from '$lib/graphql/generated';
	import DiveSiteIcon from '$lib/icons/DiveSiteIcon.svelte';

	let diveSite = {
		name: '',
		description: '',
		access: '',
		depth: 0,
		difficulty: Difficulty.Ow,
		lat: 0,
		lon: 0,
		published: false
	};

	let onSave = (site: CreateDiveSite) => {
		client.addDiveSite({ site }).then((val) => {
			goto(`/sites/${val.newDiveSite.slug}`);
		});
	};
</script>

<CheckLogin />

<div class="container grid-lg">
	<div class="columns">
		<div class="column">
			<h1 class="page-title">
				<DiveSiteIcon size="33px" /> New Dive Site
			</h1>
		</div>
	</div>
	<EditDiveSite {onSave} site={diveSite} />
</div>
