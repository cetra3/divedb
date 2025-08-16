<script>
	import { page } from '$app/state';
</script>

<svelte:head>
	<title>DiveDB - {page.status}</title>
</svelte:head>
<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<h1 class="page-title">Error - {page.status}</h1>
			<p>{page.error?.message}</p>
		</div>
	</div>
</div>
