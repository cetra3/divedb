<script type="ts">
	import { createEventDispatcher, setContext } from 'svelte';
	import L, { type LatLngBoundsExpression } from 'leaflet';

	import EventBridge from './EventBridge';

	export let options = {};
	export let fitBounds: LatLngBoundsExpression | undefined = undefined;
	export let events: any[] = [];

	let map: L.Map | undefined;

	setContext(L, {
		getMap: () => map
	});

	$: {
		if (fitBounds && map) {
			console.log('Changing bounds,', fitBounds);
			map.fitBounds(fitBounds);
		}
	}

	const dispatch = createEventDispatcher();
	let eventBridge: EventBridge;

	function initialize(container: HTMLElement) {
		map = L.map(container, options);

		if (fitBounds) {
			map.fitBounds(fitBounds);
		}

		eventBridge = new EventBridge(map, dispatch, events);
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
		<slot />
	{/if}
</div>
