<script lang="ts">
	import { setContext } from 'svelte';
	import L, { type LatLngBoundsExpression } from 'leaflet';

	import EventBridge, { type LeafletHandlers } from './EventBridge';

	interface Props {
		options?: any;
		fitBounds?: LatLngBoundsExpression | undefined;
		events?: LeafletHandlers;
		children?: import('svelte').Snippet;
	}

	let {
		options = {},
		fitBounds = undefined,
		events = {},
		children
	}: Props = $props();

	let map: L.Map | undefined = $state();

	setContext(L, {
		getMap: () => map
	});

	$effect(() => {
		if (fitBounds && map) {
			map.fitBounds(fitBounds);
		}
	});

	let eventBridge: EventBridge;

	function initialize(container: HTMLElement) {
		map = L.map(container, options);

		if (fitBounds) {
			map.fitBounds(fitBounds);
		}

		eventBridge = new EventBridge(map, events);
		return {
			destroy: () => {
				eventBridge.unregister();
				map?.remove();
			}
		};
	}

	export function getMap() {
		return map;
	}
</script>

<div style="height:100%; width:100%;" use:initialize>
	{#if map}
		{@render children?.()}
	{/if}
</div>
