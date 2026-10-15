<script lang="ts">
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

	let element: HTMLElement | undefined = $state();

	// svelte-ignore state_referenced_locally
	const popup = L.popup(options);
	const dispatch = createEventDispatcher();
	// svelte-ignore state_referenced_locally
	const eventBridge = new EventBridge(popup, dispatch, events);
	getLayer().bindPopup(popup);

	$effect(() => {
		if (element) {
			popup.setContent(element);
		}
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
