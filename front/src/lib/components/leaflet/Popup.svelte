<script type="ts">
	import { createEventDispatcher, getContext, onDestroy } from 'svelte';
	import L from 'leaflet';

	import EventBridge from './EventBridge';

	const { getLayer } = getContext<any>(L.Layer);

	export let events: any = [];
	export let options = {};

	let popup: any;
	let element: any;

	const dispatch = createEventDispatcher();
	let eventBridge: EventBridge;

	$: {
		if (!popup) {
			popup = L.popup(options);
			eventBridge = new EventBridge(popup, dispatch, events);
			getLayer().bindPopup(popup);
		}
		popup.setContent(element);
	}

	onDestroy(() => {
		eventBridge.unregister();
	});

	export function getPopup() {
		return popup;
	}
</script>

<div style="display: none;">
	<div bind:this={element}>
		<slot />
	</div>
</div>
