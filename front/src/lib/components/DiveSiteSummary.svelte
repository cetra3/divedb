<script lang="ts">
	import type { SiteSummaryMetricsFragment } from '$lib/graphql/generated';
	import SiteMetrics from './SiteMetrics.svelte';
	import DiveSiteIcon from '$lib/icons/DiveSiteIcon.svelte';
	import Photo from './photos/Photo.svelte';
	interface Props {
		site: SiteSummaryMetricsFragment;
		className?: string | undefined;
	}

	let { site, className = undefined }: Props = $props();
</script>

<div class={className ? className : 'column col-6 col-lg-12'}>
	<div class="card">
		<div class="card-image">
			<a href={`/sites/${site.slug}`}>
				{#if site.photoId}
					<Photo alt={site.name} id={site.photoId} imgClass="popular-photo" />
				{:else}
					<div class="hero bg-dark flex-center">
						<div class="hero-body">
							<DiveSiteIcon size="66px" />
						</div>
					</div>
				{/if}
			</a>
		</div>
		<div class="card-header">
			<div class="card-title h4">{site.name}</div>
			<div class="card-subtitle title-center">
				<SiteMetrics {site} />
			</div>
		</div>
		<div class="card-body">{site.summary}</div>
		<div class="card-footer">
			<a href={`/sites/${site.slug}`}>
				<button class="btn btn-primary">View {site.name} </button>
			</a>
		</div>
	</div>
</div>
