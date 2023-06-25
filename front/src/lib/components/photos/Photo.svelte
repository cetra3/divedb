<script lang="ts">
	export let id: string;
	export let imgClass: string | undefined = 'img-responsive';
	export let alt: string | undefined = undefined;
	export let width: number | undefined = undefined;
	export let height: number | undefined = undefined;
	export let lazy = true;
	export let large = false;

	const THUMB_WIDTH = 1000;
	const LARGE_WIDTH = 2000;

	let newWidth: number | undefined = undefined,
		newHeight: number | undefined = undefined;

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

	$: width !== undefined &&
		width > 0 &&
		height !== undefined &&
		height > 0 &&
		calculatedDims(width, height, large);
</script>

<picture>
	{#if large == true}
		<source type="image/webp" srcset="/api/photos/webplarge/{id}" />
		<source type="image/jpeg" srcset="/api/photos/jpeglarge/{id}" />
	{:else}
		<source type="image/webp" srcset="/api/photos/webp/{id}" />
		<source type="image/jpeg" srcset="/api/photos/jpeg/{id}" />
	{/if}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
	<img
		width={newWidth}
		height={newHeight}
		class={imgClass}
		on:click
		loading={lazy ? 'lazy' : undefined}
		src={`/api/photos/jpeg/${id}`}
		{alt}
	/>
</picture>
