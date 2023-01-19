<script lang="ts">
	export let id: string;
	export let imgClass: string | undefined = 'img-responsive';
	export let alt: string | undefined = undefined;
	export let width: number | undefined = undefined;
	export let height: number | undefined = undefined;
	export let lazy = true;

	let newWidth: number | undefined = undefined,
		newHeight: number | undefined = undefined;

	// based on a max image size of 1000 this shows the appropriate size
	const calculatedDims = (width: number, height: number) => {
		if (width > height) {
			let ratio = height / width;
			newWidth = 1000;
			newHeight = Math.round(ratio * 1000);
		} else {
			let ratio = width / height;
			newWidth = Math.round(ratio * 1000);
			newHeight = 1000;
		}
	};

	$: width !== undefined &&
		width > 0 &&
		height !== undefined &&
		height > 0 &&
		calculatedDims(width, height);
</script>

<picture>
	<source type="image/webp" srcset="/api/photos/webp/{id}" />
	<source type="image/jpeg" srcset="/api/photos/jpeg/{id}" />
	<!-- svelte-ignore a11y-click-events-have-key-events -->
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
