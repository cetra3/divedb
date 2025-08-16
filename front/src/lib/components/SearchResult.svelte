<script lang="ts">
	import type { SearchResultNodeFragment } from '$lib/graphql/generated';
	import DiveSiteIcon from '$lib/icons/DiveSiteIcon.svelte';
	import SealifeIcon from '$lib/icons/SealifeIcon.svelte';
	import Photo from './photos/Photo.svelte';

	interface Props {
		result: SearchResultNodeFragment;
	}

	let { result }: Props = $props();

	let slugLink = $derived(result.kind == 'SEALIFE' ? `/sealife/${result.slug}` : `/sites/${result.slug}`);
</script>

<div class={'column col-6 col-lg-12'}>
	<div class="card">
		<div class="card-image">
			<a href={slugLink}>
				{#if result.photoId}
					<Photo id={result.photoId} imgClass="popular-photo" alt={result.name} />
				{:else}
					<div class="hero bg-dark flex-center">
						<div class="hero-body">
							{#if result.kind == 'SEALIFE'}
								<SealifeIcon size="66px" />
							{:else}
								<DiveSiteIcon size="66px" />
							{/if}
						</div>
					</div>
				{/if}
			</a>
		</div>

		<div class="card-header">
			<div class="card-title h4">{result.name}</div>
			{#if result.scientificName}
				<div class="card-subtitle">
					{result.scientificName}
				</div>
			{/if}
		</div>
		<div class="card-body">{result.summary}</div>
		<div class="card-footer">
			<a href={slugLink}>
				<button class="btn btn-primary">View {result.name} </button>
			</a>
		</div>
	</div>
</div>
