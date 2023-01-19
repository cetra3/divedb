<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;

	let regions = data.regions;
</script>

<svelte:head>
	<title>DiveDB - Admin Settings</title>
</svelte:head>

<div class="container grid-lg">
	<div class="columns">
		<div class="column col-12 col-sm-12">
			<h1 class="page-title">Admin Settings</h1>
		</div>

		<div class="column col-12 col-sm-12">
			<div class="card">
				<div class="card-header">
					<div class="card-title h4">Regions</div>
				</div>
				<div class="card-body">
					<ul>
						{#each regions as region}
							<li>
								{region.name}
								<a href="/divesites/map/{region.slug}"><button class="btn btn-sm ">View</button></a>
								<a href={`/admin/region/edit/${region.id}`}>
									<button class="btn btn-sm">Edit</button>
								</a>
							</li>
						{/each}
					</ul>

					<a href="/admin/region/new">
						<button class="btn btn-sm btn-primary">Add New</button>
					</a>
				</div>
			</div>
		</div>
	</div>
</div>
