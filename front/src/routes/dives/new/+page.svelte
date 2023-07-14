<script lang="ts">
	import { client } from '$lib/graphql/client';
	import { goto } from '$app/navigation';
	import EditDive from '$lib/components/forms/EditDive.svelte';

	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import type { CreateDive } from '$lib/graphql/generated';
	import CheckLogin from '$lib/components/CheckLogin.svelte';

	let dive: CreateDive = {
		date: new Date(),
		depth: 0,
		duration: 0,
		description: "",
		published: false
	};

	let onSave = (dive: CreateDive) => {
		client.addDive({ dive }).then((val) => {
			goto(`/dives/${val.newDive.id}`);
		});
	};
</script>

<CheckLogin />

<div class="container grid-lg">
	<div class="columns">
		<div class="column">
			<h1 class="page-title">
				<DiveLogIcon size="33px" /> New Dive
			</h1>
		</div>
	</div>
	<EditDive {onSave} {dive} />
</div>
