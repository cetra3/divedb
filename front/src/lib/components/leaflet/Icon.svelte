<script lang="ts">
	import { getContext } from 'svelte';
	import L from 'leaflet';

	const { getMarker } = getContext<any>(L.Marker);

	export let options: any = {};

	let icon: L.Icon;

	$: {
		icon = L.icon(options);
		getMarker().setIcon(icon);
	}

	export function getIcon() {
		return icon;
	}
</script>
