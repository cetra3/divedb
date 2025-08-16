<script lang="ts">
	interface Props {
		diveId: string;
		smallOnly?: boolean;
	}

	let { diveId, smallOnly = false }: Props = $props();

	let smImage = $derived(`/api/chart/${diveId}?width=469&height=288`);
	let lgImage = $derived(`/api/chart/${diveId}?width=958&height=400`);
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
