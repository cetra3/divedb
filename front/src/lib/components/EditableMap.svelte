<script lang="ts">
	import LeafletMap from '#lib/components/leaflet/LeafletMap.svelte';
	import TileLayer from '#lib/components/leaflet/TileLayer.svelte';
	import Marker from '#lib/components/leaflet/Marker.svelte';
	import type { LeafletMouseEvent } from 'leaflet';
	interface Props {
		lat: number;
		lon: number;
		onChange: (lat: number, lon: number) => void;
	}

	let { lat = $bindable(), lon = $bindable(), onChange }: Props = $props();

	const mapOptions = {
		center: [lat, lon],
		zoom: 9
	};
	const tileUrl = 'https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png';
	const tileLayerOptions = {
		minZoom: 0,
		maxZoom: 19,
		maxNativeZoom: 19,
		attribution: '© OpenStreetMap contributors'
	};

	const onClick = (e: LeafletMouseEvent) => {
		lat = e.latlng.lat;
		lon = e.latlng.lng;
		onChange(lat, lon);
	};
</script>

<div class="editable-map-view">
	<LeafletMap events={{ click: onClick }} options={mapOptions}>
		<TileLayer url={tileUrl} options={tileLayerOptions} />
		<Marker latLng={[lat, lon]} />
	</LeafletMap>
</div>

<style global lang="scss">
	@import 'leaflet/dist/leaflet.css';

	.editable-map-view {
		height: 400px;
		.leaflet-container {
			border-radius: 0.3rem;
		}
	}
</style>
