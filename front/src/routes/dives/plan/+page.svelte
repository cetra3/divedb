<script lang="ts">
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import PlanDive from '$lib/components/forms/PlanDive.svelte';
</script>

<svelte:head>
	<title>DiveDB - Deco Planner</title>
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		<div class="column">
			<h1 class="page-title">
				<DiveLogIcon size="33px" /> Deco Planner
			</h1>
		</div>
	</div>
	<PlanDive />
</div>
