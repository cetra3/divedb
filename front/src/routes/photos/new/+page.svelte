<script context="module">
</script>

<script lang="ts">
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	import ImageUpload from '$lib/components/forms/ImageUpload.svelte';
	import ImageList from '$lib/components/ImageList.svelte';
	import type { PhotoSummaryFragment } from '$lib/graphql/generated';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';

	let isDragging = false;

	let filesToUpload: (File | undefined)[] = [];

	let photosUploaded: PhotoSummaryFragment[] = [];

	const clickUpload = () => {
		document.getElementById('fileupload')?.click();
	};

	const onUploaded = (e: CustomEvent<{ photo: PhotoSummaryFragment; index: number }>) => {
		console.log(e.detail);

		const { photo, index } = e.detail;

		filesToUpload[index] = undefined;

		photosUploaded = [photo, ...photosUploaded];
	};

	const onUpload = (e: Event) => {
		let files = (e.target as any).files;
		if (files) {
			onAddFiles(files);
		}
	};

	const onAddFiles = (files: FileList) => {
		let newFiles = [];
		for (let i = 0; i < files.length; i++) {
			let file = files.item(i);

			if (file) {
				newFiles.push(file);
			}
		}

		filesToUpload = [...filesToUpload, ...newFiles];
	};
</script>

<svelte:head>
	<title>DiveDB - Add new photos</title>
</svelte:head>

<CheckLogin />

<div class="dive-sites container grid-lg">
	<div class="columns">
		<div class="column col-12 col-lg-12">
			<h1 class="page-title">
				<PhotoIcon size="22px" /> Upload Photos
				{#if filesToUpload.length != photosUploaded.length}
					<span class="margin-left-30 loading loading-lg" />
				{:else if filesToUpload.length > 0}
					<i class="icon icon-2x icon-check" />
				{/if}
			</h1>
		</div>

		<div class={`column  col-12`}>
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div
				class="card pointer"
				on:click={clickUpload}
				on:dragenter={() => (isDragging = true)}
				on:dragleave={() => (isDragging = false)}
				on:dragover={(e) => {
					e.preventDefault();
					e.stopPropagation();
					isDragging = true;
				}}
				on:drop={(e) => {
					e.preventDefault();
					e.stopPropagation();
					isDragging = false;

					const files = e.dataTransfer?.files;

					if (files) {
						onAddFiles(files);
					}
				}}
			>
				<input
					on:change={onUpload}
					id="fileupload"
					type="file"
					multiple
					accept=".jpg,.jpeg"
					hidden
				/>
				<div class:dragging={isDragging} class="card-body hero flex-center">
					<div class="hero-body">
						<h5>Drag and Drop here to Upload photos</h5>
					</div>
					<div class="hero-body">
						<PhotoIcon size="66px" />
					</div>
					<button class="btn" class:btn-primary={!isDragging}>Or Click Here to Upload</button>
				</div>
			</div>
		</div>
		{#each filesToUpload as file, index}
			{#if file}
				<ImageUpload {index} on:upload={onUploaded} {file} />
			{/if}
		{/each}
	</div>
	<ImageList photos={photosUploaded} />
</div>
