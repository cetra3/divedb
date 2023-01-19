<script type="ts">
	export let size: string;
	export let viewBox = '0 0 16 16';
</script>

<svg width={size} height={size} {viewBox} fill="currentColor" xmlns="http://www.w3.org/2000/svg">
	<slot />
</svg>
