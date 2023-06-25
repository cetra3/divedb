<script lang="ts">
	import LeafletMap from '$lib/components/leaflet/LeafletMap.svelte';
	import TileLayer from '$lib/components/leaflet/TileLayer.svelte';
	import Marker from '$lib/components/leaflet/Marker.svelte';
	export let lat: number;
	export let lon: number;
	export let onChange: (lat: number, lon: number) => void;

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

	const onClick = (e: any) => {
		lat = e.detail.latlng.lat;
		lon = e.detail.latlng.lng;
		onChange(lat, lon);
	};
</script>

<div class="editable-map-view">
	<LeafletMap events={['click']} on:click={onClick} options={mapOptions}>
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
