<script lang="ts">
	import type { SiteSummaryMetricsFragment } from '$lib/graphql/generated';
	import type { LatLngBoundsExpression } from 'leaflet';
	import DiveSiteSummary from './DiveSiteSummary.svelte';
	import LeafletMap from '$lib/components/leaflet/LeafletMap.svelte';
	import TileLayer from '$lib/components/leaflet/TileLayer.svelte';
	import Marker from '$lib/components/leaflet/Marker.svelte';
	import Icon from '$lib/components/leaflet/Icon.svelte';
	import Popup from '$lib/components/leaflet/Popup.svelte';
	export let sites: SiteSummaryMetricsFragment[];
	export let selectedSite: string | undefined = undefined;
	export let bounds: LatLngBoundsExpression | undefined = undefined;

	let showOnlyDived = true;

	const sitesToDisplay = (showOnlyDived: boolean) => {
		return sites
			.filter((site) => site.lat != 0 && site.lon != 0)
			.filter((val) => {
				if (selectedSite && val.slug === selectedSite) {
					return true;
				}

				if (showOnlyDived && val.siteMetrics.diveCount == 0) {
					return false;
				}

				return true;
			});
	};

	$: displaySites = sitesToDisplay(showOnlyDived);

	const siteToSelect = sites.find((site) => site.slug === selectedSite);

	const center =
		siteToSelect !== undefined
			? [siteToSelect.lat, siteToSelect.lon]
			: // We started in South Australia, so should the map!
			  [-34.696, 137.726];

	const mapOptions = {
		center,
		zoom: siteToSelect != undefined ? 11 : 8
	};
	const tileUrl = 'https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png';
	const tileLayerOptions = {
		minZoom: 0,
		maxZoom: 19,
		maxNativeZoom: 19,
		attribution: '© OpenStreetMap contributors'
	};

	let selectedPopup;
	let selectedMarker;

	const iconOptions = {
		iconUrl: '/leaflet/marker-icon-2x-blue.png',
		shadowUrl: '/leaflet/marker-shadow.png',
		iconSize: [25, 41],
		iconAnchor: [12, 41],
		popupAnchor: [1, -34],
		shadowSize: [41, 41]
	};

	const selectedIconOptions = {
		...iconOptions,
		iconUrl: '/leaflet/marker-icon-2x-red.png'
	};

	const notDivedIconOptions = {
		...iconOptions,
		iconUrl: '/leaflet/marker-icon-2x-grey.png'
	};
</script>

<div class="map-view-container">
	<div class="map-view">
		<div class="map-options">
			<label class="form-checkbox pointer">
				<input type="checkbox" bind:checked={showOnlyDived} />
				<i class="form-icon" /> Show only sites with logged dives
			</label>
		</div>
		<LeafletMap options={mapOptions} fitBounds={bounds}>
			<TileLayer url={tileUrl} options={tileLayerOptions} />
			{#each displaySites as site}
				<Marker latLng={[site.lat, site.lon]}>
					<Icon options={site.siteMetrics.diveCount > 0 ? iconOptions : notDivedIconOptions} />
					<Popup>
						<DiveSiteSummary className="site-summary" {site} />
					</Popup>
				</Marker>
			{/each}
			{#if siteToSelect}
				<Marker bind:this={selectedMarker} latLng={[siteToSelect.lat, siteToSelect.lon]}>
					<Icon options={selectedIconOptions} />
					<Popup bind:this={selectedPopup}>
						<DiveSiteSummary className="site-summary" site={siteToSelect} />
					</Popup>
				</Marker>
			{/if}
		</LeafletMap>
	</div>
</div>

<style global lang="scss">
	@import 'leaflet/dist/leaflet.css';
	.map-view {
		position: absolute;
		top: 0;
		bottom: 0;
		width: 100%;
	}

	.map-options {
		position: absolute;
		top: 0;
		right: 0;
		z-index: 500;
		border-radius: 0 0 0 0.3rem;
		background-color: rgba(255, 255, 255, 0.8);
		padding: 0 10px;
	}

	.leaflet-container {
		border-radius: 0.3rem 0.3rem 0 0;
	}

	.leaflet-container .leaflet-control-attribution {
		border-radius: 0.3rem 0 0 0;
	}

	.map-view-container {
		position: relative;
		flex: 1;
	}

	.leaflet-popup-content-wrapper {
		padding: 0 !important;
	}

	.leaflet-popup-content {
		margin: 0 !important;
		.card {
			border: none;
			margin-bottom: 0;
			font-family: 'Asap', -apple-system, system-ui, BlinkMacSystemFont, 'Segoe UI', Roboto;
			font-size: 16px;
			color: #3b4351;
		}
	}
</style>
