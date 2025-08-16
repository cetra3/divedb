<script lang="ts">
	import type { SiteSummaryMetricsFragment } from '$lib/graphql/generated';
	import DiveLogIcon from '$lib/icons/DiveLogIcon.svelte';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';
	interface Props {
		site: SiteSummaryMetricsFragment;
	}

	let { site }: Props = $props();
</script>

<span title={`${site.siteMetrics.photoCount} photos taken at ${site.name}`} class="title-center">
	<PhotoIcon size="22px" />
	<span class="site-metrics">
		{site.siteMetrics.photoCount} Photos,
	</span>
</span>
<span title={`${site.siteMetrics.diveCount} dives logged at ${site.name}`} class="title-center">
	<DiveLogIcon size="22px" />
	<span class="site-metrics">
		{site.siteMetrics.diveCount} Dives Logged
	</span>
</span>
