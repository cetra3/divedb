<script lang="ts">
	import { run } from 'svelte/legacy';

	import { createEventDispatcher, getContext, onDestroy } from 'svelte';
	import L from 'leaflet';

	import EventBridge from './EventBridge';

	const { getMap } = getContext<any>(L);

	interface Props {
		url: any;
		wms?: boolean;
		opacity?: number;
		zIndex?: number;
		options?: any;
		events?: any;
	}

	let {
		url,
		wms = false,
		opacity = 1.0,
		zIndex = 1,
		options = {},
		events = []
	}: Props = $props();

	let tileLayer: any = $state();

	const dispatch = createEventDispatcher();
	let eventBridge: EventBridge = $state();

	run(() => {
		if (!tileLayer) {
			tileLayer = (!wms ? L.tileLayer(url, options) : L.tileLayer.wms(url, options)).addTo(
				getMap()
			);
			eventBridge = new EventBridge(tileLayer, dispatch, events);
		}
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
