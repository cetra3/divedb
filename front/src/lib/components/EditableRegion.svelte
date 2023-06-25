<script lang="ts">
	import LeafletMap from '$lib/components/leaflet/LeafletMap.svelte';
	import TileLayer from '$lib/components/leaflet/TileLayer.svelte';
	import { centerVals } from '$lib/util/bounds';
	import type { LatLngBoundsExpression } from 'leaflet';
	import DrawRectangle from './leaflet/DrawRectangle.svelte';
	export let bounds: LatLngBoundsExpression | undefined = undefined;

	// Don't change when editing
	let initialBounds = bounds;

	const mapOptions = {
		center: bounds ? centerVals(bounds) : [-34.432286, 137.460938],
		zoom: 5
	};
	const tileUrl = 'https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png';
	const tileLayerOptions = {
		minZoom: 0,
		maxZoom: 19,
		maxNativeZoom: 19,
		attribution: '© OpenStreetMap contributors'
	};
</script>

<div class="editable-map-view">
	<LeafletMap options={mapOptions} fitBounds={initialBounds}>
		<TileLayer url={tileUrl} options={tileLayerOptions} />
		<DrawRectangle bind:bounds />
	</LeafletMap>
</div>

<style global lang="scss">
	@import 'leaflet/dist/leaflet.css';

	.editable-map-view {
		height: 600px;
		.leaflet-container {
			border-radius: 0.3rem;
		}
	}
</style>
