<script lang="ts">
	interface Props {
		size: string;
		viewBox?: string;
		children?: import('svelte').Snippet;
	}

	let { size, viewBox = '0 0 16 16', children }: Props = $props();
</script>

<svg width={size} height={size} {viewBox} fill="currentColor" xmlns="http://www.w3.org/2000/svg">
	{@render children?.()}
</svg>
