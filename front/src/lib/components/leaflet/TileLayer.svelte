<script lang="ts">
	import { getContext, onDestroy } from 'svelte';
	import L from 'leaflet';

	import EventBridge, { type LeafletHandlers } from './EventBridge';

	const { getMap } = getContext<any>(L);

	interface Props {
		url: any;
		wms?: boolean;
		opacity?: number;
		zIndex?: number;
		options?: any;
		events?: LeafletHandlers;
	}

	let {
		url,
		wms = false,
		opacity = 1.0,
		zIndex = 1,
		options = {},
		events = {}
	}: Props = $props();

	// LeafletMap only renders its children once the map exists, so create the layer up front
	// svelte-ignore state_referenced_locally
	const tileLayer = (!wms ? L.tileLayer(url, options) : L.tileLayer.wms(url, options)).addTo(
		getMap()
	);

	// svelte-ignore state_referenced_locally
	const eventBridge = new EventBridge(tileLayer, events);

	$effect.pre(() => {
		tileLayer.setUrl(url);
		tileLayer.setOpacity(opacity);
		tileLayer.setZIndex(zIndex);
	});

	onDestroy(() => {
		eventBridge.unregister();
		tileLayer.removeFrom(getMap());
	});

	export function getTileLayer() {
		return tileLayer;
	}
</script>
