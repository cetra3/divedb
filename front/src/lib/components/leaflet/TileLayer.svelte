<script lang="ts">
	import { createEventDispatcher, getContext, onDestroy } from 'svelte';
	import L from 'leaflet';

	import EventBridge from './EventBridge';

	const { getMap } = getContext<any>(L);

	export let url: any;
	export let wms = false;
	export let opacity = 1.0;
	export let zIndex = 1;
	export let options = {};
	export let events: any = [];

	let tileLayer: any;

	const dispatch = createEventDispatcher();
	let eventBridge: EventBridge;

	$: {
		if (!tileLayer) {
			tileLayer = (!wms ? L.tileLayer(url, options) : L.tileLayer.wms(url, options)).addTo(
				getMap()
			);
			eventBridge = new EventBridge(tileLayer, dispatch, events);
		}
		tileLayer.setUrl(url);
		tileLayer.setOpacity(opacity);
		tileLayer.setZIndex(zIndex);
	}

	onDestroy(() => {
		eventBridge.unregister();
		tileLayer.removeFrom(getMap());
	});

	export function getTileLayer() {
		return tileLayer;
	}
</script>
