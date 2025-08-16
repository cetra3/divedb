<script lang="ts">
	import { run, createBubbler } from 'svelte/legacy';

	const bubble = createBubbler();
	interface Props {
		id: string;
		imgClass?: string | undefined;
		alt?: string | undefined;
		width?: number | undefined;
		height?: number | undefined;
		lazy?: boolean;
		large?: boolean;
	}

	let {
		id,
		imgClass = 'img-responsive',
		alt = undefined,
		width = undefined,
		height = undefined,
		lazy = true,
		large = false
	}: Props = $props();

	const THUMB_WIDTH = 1000;
	const LARGE_WIDTH = 2000;

	let newWidth: number | undefined = $state(undefined),
		newHeight: number | undefined = $state(undefined);

	// based on a max image size of 1000 this shows the appropriate size
	const calculatedDims = (width: number, height: number, useLarge: boolean) => {
		let desiredWidth = useLarge ? LARGE_WIDTH : THUMB_WIDTH;

		if (width > height) {
			let ratio = height / width;
			newWidth = desiredWidth;
			newHeight = Math.round(ratio * desiredWidth);
		} else {
			let ratio = width / height;
			newWidth = Math.round(ratio * desiredWidth);
			newHeight = desiredWidth;
		}
	};

	run(() => {
		width !== undefined &&
			width > 0 &&
			height !== undefined &&
			height > 0 &&
			calculatedDims(width, height, large);
	});
</script>

<picture>
	{#if large == true}
		<source type="image/webp" srcset="/api/photos/webplarge/{id}" />
		<source type="image/jpeg" srcset="/api/photos/jpeglarge/{id}" />
	{:else}
		<source type="image/webp" srcset="/api/photos/webp/{id}" />
		<source type="image/jpeg" srcset="/api/photos/jpeg/{id}" />
	{/if}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<img
		width={newWidth}
		height={newHeight}
		class={imgClass}
		onclick={bubble('click')}
		loading={lazy ? 'lazy' : undefined}
		src={`/api/photos/jpeg/${id}`}
		{alt}
	/>
</picture>
