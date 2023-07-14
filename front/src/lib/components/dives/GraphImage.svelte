<script lang="ts">
	export let diveId: string;
	export let smallOnly = false;

	$: smImage = `/api/chart/${diveId}?width=469&height=288`;
	$: lgImage = `/api/chart/${diveId}?width=958&height=400`;
</script>

<picture>
	{#if !smallOnly}
		<source media="(max-width: 959px)" width={469} height={288} srcset={smImage} />
		<source media="(min-width: 960px)" width={958} height={400} srcset={lgImage} />
	{/if}
	<img
		loading="lazy"
		width={469}
		height={288}
		src={smImage}
		class="img-responsive"
		alt={'Dive Chart'}
	/>
</picture>
