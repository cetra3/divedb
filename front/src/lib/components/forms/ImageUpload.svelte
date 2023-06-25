<script context="module" type="ts">
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

	import { createEventDispatcher, onDestroy, onMount } from 'svelte';

	export let file: File;
	export let index: number;

	let uploading = false;
	const dispatch = createEventDispatcher();

	let completed = false;

	let dataUrl: string | undefined = undefined;

	let error: string | undefined = undefined;

	let percentComplete = 0;

	onMount(() => {
		dataUrl = window.URL.createObjectURL(file);
		uploadFile(file);
	});

	onDestroy(() => {
		if (dataUrl) {
			window.URL.revokeObjectURL(dataUrl);
		}
	});

	const uploadFile = (file: File) => {
		uploading = true;
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
			error = request.responseText;
			uploading = false;
		});

		request.addEventListener('load', () => {
			if (request.status == 200) {
				uploading = false;

				console.log(request.responseText);

				let photoArray = JSON.parse(request.responseText);

				let photo: UploadResponse = photoArray[0];

				client.getPhotos({ id: photo.id }).then((val) => {
					const newPhoto = val.photos[0];
					dispatch('upload', {
						photo: newPhoto,
						index
					});
					completed = true;
				});
			} else {
				error = request.responseText;
			}
		});

		request.open('POST', '/api/photos');
		request.setRequestHeader('DiveDB-Token', localStorage.getItem('token') ?? '');
		request.send(formData);
	};
</script>

{#if dataUrl}
	<div class="column col-3 col-sm-6">
		<div class="img-holder">
			<!-- svelte-ignore a11y-missing-attribute -->
			<img src={dataUrl} class:incomplete={!completed} class="img-edit img-responsive" />
			<div class="img-overlay" style={`margin-left:${percentComplete}%`} />
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
