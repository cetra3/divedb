<script lang="ts">
	import { run } from 'svelte/legacy';

	import { createEventDispatcher, getContext, onDestroy, setContext } from 'svelte';
	import L from 'leaflet';

	import EventBridge from './EventBridge';

	const { getMap } = getContext<any>(L);

	const defaultIcon = L.icon({
		iconUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.7.1/images/marker-icon.png',
		iconRetinaUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.7.1/images/marker-icon-2x.png',
		shadowUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.7.1/images/marker-shadow.png',
		iconSize: [25, 41],
		iconAnchor: [12, 41],
		popupAnchor: [1, -34],
		tooltipAnchor: [16, -28],
		shadowSize: [41, 41]
	});

	interface Props {
		latLng: any;
		zIndexOffset?: number;
		icon?: any;
		opacity?: number;
		options?: any;
		events?: any[];
		children?: import('svelte').Snippet;
	}

	let {
		latLng,
		zIndexOffset = 0,
		icon = defaultIcon,
		opacity = 1.0,
		options = {},
		events = [],
		children
	}: Props = $props();

	let marker: L.Marker = $state();

	setContext(L.Layer, {
		getLayer: () => marker
	});
	setContext(L.Marker, {
		getMarker: () => marker
	});

	const dispatch = createEventDispatcher();
	let eventBridge: EventBridge = $state();

	run(() => {
		if (!marker) {
			marker = L.marker(latLng, options).addTo(getMap());
			eventBridge = new EventBridge(marker, dispatch, events);
		}
		marker.setLatLng(latLng);
		marker.setZIndexOffset(zIndexOffset);
		marker.setIcon(icon);
		marker.setOpacity(opacity);
	});

	onDestroy(() => {
		eventBridge.unregister();
		marker.removeFrom(getMap());
	});

	export function getMarker() {
		return marker;
	}
</script>

<div>
	{#if marker}
		{@render children?.()}
	{/if}
</div>
