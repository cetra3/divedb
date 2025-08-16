<script module lang="ts">
	export interface UploadResponse {
		id: string;
		user_id: string;
		filename: string;
		date?: string;
		dive_id?: string;
		dive_site_id?: string;
		size: number;
		upload_date: string;
		hide_location: boolean;
	}
</script>

<script lang="ts">
	import { client } from '$lib/graphql/client';
	import type { PhotoSummaryFragment } from '$lib/graphql/generated';

	import { onDestroy, onMount } from 'svelte';

	interface Props {
		file: File;
		index: number;
		internal?: boolean;
		doUpload?: boolean;
		onUpload: (response: { photo: PhotoSummaryFragment; index: number }) => void;
		onError: (error: string) => void;
	}

	let { file, index, internal = false, doUpload = true, onError, onUpload }: Props = $props();

	let completed = $state(false);

	let dataUrl: string | undefined = $state(undefined);

	let error: string | undefined = $state(undefined);

	let percentComplete = $state(0);

	$effect(() => {
		if (doUpload) {
			uploadFile(file);
		}
	});

	onMount(() => {
		dataUrl = window.URL.createObjectURL(file);
	});

	onDestroy(() => {
		if (dataUrl) {
			window.URL.revokeObjectURL(dataUrl);
		}
	});

	const setError = (errorString: string) => {
		if (errorString == '') {
			error = 'An unexpected error occurred';
		} else {
			error = errorString;
		}
		onError(error);
	};

	const uploadFile = (file: File) => {
		let formData = new FormData();

		if (file) {
			formData.append(`file`, file);
		}

		let request = new XMLHttpRequest();

		request.overrideMimeType('text/plain');

		request.upload.addEventListener('progress', (ev) => {
			if (ev.lengthComputable) {
				percentComplete = (ev.loaded / ev.total) * 100;
			}
		});

		request.addEventListener('error', () => {
			setError(request.responseText);
		});

		request.addEventListener('load', () => {
			if (request.status == 200) {
				let photoArray = JSON.parse(request.responseText);

				let photo: UploadResponse = photoArray[0];

				client.getPhotos({ id: photo.id }).then((val) => {
					const newPhoto = val.photos[0];
					onUpload({
						photo: newPhoto,
						index
					});
					completed = true;
				});
			} else {
				setError(request.responseText);
			}
		});

		request.open('POST', `/api/photos?internal=${internal}`);
		request.setRequestHeader('DiveDB-Token', localStorage.getItem('token') ?? '');
		request.send(formData);
	};
</script>

{#if error}
	<div class="column col-12">{error}</div>
{:else if dataUrl}
	<div class="column col-3 col-sm-6">
		<div class="img-holder">
			<!-- svelte-ignore a11y_missing_attribute -->
			<img src={dataUrl} class:incomplete={!completed} class="img-edit img-responsive" />
			<div class="img-overlay" style={`margin-left:${percentComplete}%`}></div>
		</div>
	</div>
{/if}

<style global lang="scss">
	.img-hidden {
		display: none;
	}
	.img-holder {
		display: inline-block;
		position: relative;
		img {
			transition: opacity 500ms;
		}
	}
	.incomplete {
		opacity: 0.6;
	}
	.img-overlay {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: white;
		margin-left: 50%;
		transition: margin 500ms;
		animation: animate 3s ease-out infinite;
	}

	@keyframes animate {
		from {
			opacity: 0.3;
		}
		50% {
			opacity: 0.5;
		}
		to {
			opacity: 0.3;
		}
	}
</style>
