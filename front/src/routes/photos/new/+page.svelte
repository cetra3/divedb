<script lang="ts">
	import CheckLogin from '$lib/components/CheckLogin.svelte';
	import ImageUpload from '$lib/components/forms/ImageUpload.svelte';
	import ImageList from '$lib/components/ImageList.svelte';
	import type { PhotoSummaryFragment } from '$lib/graphql/generated';
	import PhotoIcon from '$lib/icons/PhotoIcon.svelte';

	let isDragging = $state(false);

	interface QueuedFile {
		file: File;
		shouldUpload: boolean;
	}

	let filesToQueue: (QueuedFile | undefined)[] = $state([]);

	let photosUploaded: PhotoSummaryFragment[] = $state([]);

	let hasErrors = $state(false);

	const clickUpload = () => {
		document.getElementById('fileupload')?.click();
	};

	const onUpload = ({ photo, index }: { photo: PhotoSummaryFragment; index: number }) => {
		let newFilesToQueue = [...filesToQueue];

		newFilesToQueue[index] = undefined;

		// if there is any outstanding files to upload, start them now
		let nextToUpload = newFilesToQueue.find((file) => file?.shouldUpload == false);
		if (nextToUpload) {
			nextToUpload.shouldUpload = true;
		}

		filesToQueue = [...newFilesToQueue];
		photosUploaded = [photo, ...photosUploaded];
	};

	const onError = () => {
		hasErrors = true;
	};

	const onFileChange = (e: Event) => {
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
			    // limit the number of uploads to 5 at a time
				let shouldUpload: boolean =
					newFiles.length + filesToQueue.filter((val) => val !== undefined).length < 5;
				newFiles.push({ file, shouldUpload });
			}
		}

		filesToQueue = [...filesToQueue, ...newFiles];
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
				{#if !hasErrors}
					{#if filesToQueue.length != photosUploaded.length}
						<span class="margin-left-30 loading loading-lg"></span>
					{:else if filesToQueue.length > 0}
						<i class="icon icon-2x icon-check"></i>
					{/if}
				{/if}
			</h1>
		</div>

		<div class={`column  col-12`}>
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				class="card pointer"
				onclick={clickUpload}
				ondragenter={() => (isDragging = true)}
				ondragleave={() => (isDragging = false)}
				ondragover={(e) => {
					e.preventDefault();
					e.stopPropagation();
					isDragging = true;
				}}
				ondrop={(e) => {
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
					onchange={onFileChange}
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
		{#each filesToQueue as file, index}
			{#if file}
				<ImageUpload doUpload={file.shouldUpload} {index} {onUpload} {onError} file={file.file} />
			{/if}
		{/each}
	</div>
	<ImageList photos={photosUploaded} />
</div>
