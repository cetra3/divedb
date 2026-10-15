<script lang="ts">
	import { getContext } from 'svelte';
	import L from 'leaflet';

	const { getMarker } = getContext<any>(L.Marker);

	interface Props {
		options?: any;
	}

	let { options = {} }: Props = $props();

	let icon: L.Icon;

	$effect.pre(() => {
		icon = L.icon(options);
		getMarker().setIcon(icon);
	});

	export function getIcon() {
		return icon;
	}
</script>
