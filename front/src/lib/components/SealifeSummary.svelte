<script lang="ts">
	import type { SealifeSummaryFragment } from '$lib/graphql/generated';
	import SealifeIcon from '$lib/icons/SealifeIcon.svelte';
	import Photo from './photos/Photo.svelte';
	interface Props {
		sealife: SealifeSummaryFragment;
		className?: string | undefined;
	}

	let { sealife, className = undefined }: Props = $props();
</script>

<div class={className ? className : 'column col-6 col-lg-12'}>
	<div class="card">
		<div class="card-image">
			<a href={`/sealife/${sealife.slug}`}>
				{#if sealife.photoId}
					<Photo alt={sealife.name} id={sealife.photoId} />
				{:else}
					<div class="hero bg-dark flex-center">
						<div class="hero-body">
							<SealifeIcon size="66px" />
						</div>
					</div>
				{/if}
			</a>
		</div>
		<div class="card-header">
			<div class="card-title h4">{sealife.name}</div>
			{#if sealife.scientificName}
				<div class="card-subtitle title-center">
					{sealife.scientificName}
				</div>
			{/if}
		</div>
		<div class="card-body">{sealife.summary}</div>
		<div class="card-footer">
			<a href={`/sealife/${sealife.slug}`}>
				<button class="btn btn-primary">View {sealife.name} </button>
			</a>
		</div>
	</div>
</div>
