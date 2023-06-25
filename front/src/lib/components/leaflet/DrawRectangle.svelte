<script lang="ts">
	import { getContext, onDestroy, onMount } from 'svelte';
	import L, { type LatLngBoundsExpression } from 'leaflet';
	import '@geoman-io/leaflet-geoman-free';
	import '@geoman-io/leaflet-geoman-free/dist/leaflet-geoman.css';

	const { getMap } = getContext<any>(L);
	let layerGroup: L.LayerGroup;
	let map: L.Map;

	export let bounds: LatLngBoundsExpression | undefined = undefined;

	onMount(() => {
		map = getMap();
		layerGroup = new L.LayerGroup().addTo(map);

		const addRect = (rect: L.Rectangle) => {
			layerGroup.addLayer(rect);
			rect.on('pm:change', () => {
				bounds = rect.getBounds();
			});
		};

		if (bounds) {
			let rect = new L.Rectangle(bounds);
			addRect(rect);
		}

		map.pm.setGlobalOptions({ layerGroup });

		map.pm.addControls({
			position: 'topleft',
			drawMarker: false,
			drawCircle: false,
			drawPolygon: false,
			drawPolyline: false,
			drawCircleMarker: false,
			drawText: false,
			cutPolygon: false,
			oneBlock: true,
			removalMode: false,
			rotateMode: false
		});

		map.on('pm:create', (e) => {
			layerGroup.clearLayers();
			if (e.layer instanceof L.Rectangle) {
				bounds = e.layer.getBounds();
				addRect(e.layer);
			}
		});
	});

	onDestroy(() => {
		map.pm.removeControls();

		map.removeLayer(layerGroup);
	});
</script>
