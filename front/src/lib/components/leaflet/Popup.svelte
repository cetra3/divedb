<script lang="ts">
	import { run } from 'svelte/legacy';

	import { createEventDispatcher, getContext, onDestroy } from 'svelte';
	import L from 'leaflet';

	import EventBridge from './EventBridge';

	const { getLayer } = getContext<any>(L.Layer);

	interface Props {
		events?: any;
		options?: any;
		children?: import('svelte').Snippet;
	}

	let { events = [], options = {}, children }: Props = $props();

	let popup: any = $state();
	let element: any = $state();

	const dispatch = createEventDispatcher();
	let eventBridge: EventBridge = $state();

	run(() => {
		if (!popup) {
			popup = L.popup(options);
			eventBridge = new EventBridge(popup, dispatch, events);
			getLayer().bindPopup(popup);
		}
		popup.setContent(element);
	});

	onDestroy(() => {
		eventBridge.unregister();
	});

	export function getPopup() {
		return popup;
	}
</script>

<div style="display: none;">
	<div bind:this={element}>
		{@render children?.()}
	</div>
</div>
