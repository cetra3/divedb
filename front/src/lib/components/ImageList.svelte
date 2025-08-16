<!-- @migration-task Error while migrating Svelte code: This migration would change the name of a slot (photo-label to photo_label) making the component unusable -->
<script lang="ts">
	import type { GetPhotosQueryVariables, PhotoSummaryFragment } from '$lib/graphql/generated';
	import { client } from '$lib/graphql/client';
	import { throttle } from 'lodash-es';
	export let query: GetPhotosQueryVariables | undefined = undefined;
	export let condensed = false;
	export let photos: PhotoSummaryFragment[];

	import { browser } from '$app/environment';

	import { Swiper, SwiperSlide } from 'swiper/svelte';

	import type { Swiper as SwiperVar } from 'swiper';
	import { Zoom, Navigation, Pagination, Virtual } from 'swiper';

	import 'swiper/css';

	import 'swiper/css/zoom';
	import 'swiper/css/navigation';
	import 'swiper/css/pagination';
	import PhotoLabels from './photos/PhotoLabels.svelte';
	import imageAlt from '$lib/util/imageAlt';
	import PhotoSlide from './photos/PhotoSlide.svelte';
	import Photo from './photos/Photo.svelte';
	import { onDestroy } from 'svelte';

	let scrollPercent = 0;
	let loading = false;
	let atTheEnd = false;

	let curIdx = 0;

	let showModal = false;

	let swiper: SwiperVar | undefined = undefined;

	let toEdit: PhotoSummaryFragment | undefined = undefined;

	$: offset = photos.length;

	let photoId: string | undefined = undefined;

	const onEditSave = (newPhoto: PhotoSummaryFragment) => {
		const idx = photos.findIndex((photo) => photo.id == newPhoto.id);

		photos[idx] = newPhoto;

		toEdit = undefined;

		if (swiper) {
			setTimeout(() => swiper?.virtual.update(true));
		}
	};

	const closeModal = () => {
		if (swiper) {
			swiper.destroy();
			swiper = undefined;
		}

		const body = document.querySelector('body');
		if (body) {
			body.style.overflow = 'visible';
		}

		toEdit = undefined;
		showModal = false;
	};

	const updateSwiper = () => {
		if (swiper && photoId) {
			const idx = photos.findIndex((photo) => photo.id == photoId);
			swiper.slideTo(idx);
			setTimeout(() => swiper?.update());
		}
	};

	const morePhotos = () => {
		if (query !== undefined && !loading && !atTheEnd) {
			loading = true;
			client.getPhotos({ ...query, offset }).then((val) => {
				atTheEnd = val.photos.length == 0;
				photos = [...photos, ...val.photos];
				loading = false;
				if (swiper) {
					swiper.virtual.slides = photos;
					swiper.virtual.update(true);
				}
			});
		}
	};

	onDestroy(() => {
		if (browser) {
			const body = document.querySelector('body');
			if (body) {
				body.style.overflow = 'visible';
			}
		}
	});

	const handleScroll = throttle(() => {
		if (!condensed) {
			let scrollTop = window.scrollY;
			let docHeight = document.body.offsetHeight;
			let winHeight = window.innerHeight;
			scrollPercent = scrollTop / (docHeight - winHeight);

			if (scrollPercent > 0.5) {
				morePhotos();
			}
		}
	}, 300);
</script>

{#if showModal}
	<div class="photo-modal">
		<!-- svelte-ignore a11y-missing-content -->
		<!-- svelte-ignore a11y-invalid-attribute -->
		<div class="modal-close">
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<span class="pointer" on:click={closeModal}>✕ </span>
		</div>
		<Swiper
			style="--swiper-navigation-color: #FFF;--swiper-pagination-color: #FFF"
			zoom
			navigation
			pagination={{
				dynamicBullets: true,
				clickable: true
			}}
			virtual={{ slides: photos }}
			on:swiper={(val) => {
				// @ts-ignore
				swiper = val.detail[0];
				updateSwiper();
			}}
			on:slideChange={(val) => {
				if (swiper) {
					curIdx = swiper.activeIndex;
				}
				toEdit = undefined;
			}}
			on:reachEnd={morePhotos}
			let:virtualData={{ slides, offset, from }}
			modules={[Zoom, Navigation, Pagination, Virtual]}
		>
			{#each slides as slide, index (from + index)}
				<SwiperSlide virtualIndex={from + index} style={`left: ${offset}px`}>
					<div class="swiper-zoom-container">
						<Photo
							large={true}
							width={slide.width}
							height={slide.height}
							alt={imageAlt(slide)}
							id={slide.id}
						/>
					</div>
				</SwiperSlide>
			{/each}
		</Swiper>
		<div class="photo-modal-toolbar">
			<PhotoLabels photo={photos[curIdx]}>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<span
					class="pointer label label-secondary"
					on:click={() => {
						if (toEdit != undefined) {
							toEdit = undefined;
						} else {
							toEdit = photos[curIdx];
						}
					}}>Edit</span
				>
				<slot name="photo-label" photo={photos[curIdx].id} />
			</PhotoLabels>

			{#if toEdit}
				<PhotoSlide {onEditSave} photo={toEdit} />
			{/if}
		</div>
	</div>
{/if}

<svelte:window on:scroll={handleScroll} />
{#if !condensed}
	<div class="columns" class:hide-scroll={showModal}>
		{#each photos as photo, idx}
			<div class="column col-6 col-sm-12">
				<div class="card">
					<div class="card-image">
						<Photo
							alt={imageAlt(photo)}
							width={photo.width}
							height={photo.height}
							imgClass="img-responsive pointer"
							id={photo.id}
							on:click={() => {
								photoId = photo.id;
								updateSwiper();
								showModal = true;
								const body = document.querySelector('body');
								if (body) {
									body.style.overflow = 'hidden';
								}
							}}
						/>
					</div>
					<div class="card-footer photo-labels no-padding">
						<PhotoLabels {photo} />
					</div>
				</div>
			</div>
		{/each}
	</div>
{:else}
	<div class="condensed">
		{#each photos as photo}
			<div class="condensed-photo">
				<Photo
					on:click={() => {
						photoId = photo.id;
						updateSwiper();
						showModal = true;
						const body = document.querySelector('body');
						if (body) {
							body.style.overflow = 'hidden';
						}
					}}
					imgClass="pointer"
					id={photo.id}
				/>
			</div>
		{/each}
		{#if photos.length % 3 != 0}
			<div class="condensed-spacer"></div>
		{/if}
	</div>
{/if}

<style global lang="scss">
	.hide-scroll {
		overflow: hidden;
	}

	.condensed {
		display: flex;
		font-size: 0;
		flex-wrap: wrap;
		margin-bottom: 0.4rem;

		.condensed-photo {
			flex-grow: 1;
			height: 100px;
			img {
				border-radius: 0.3rem;
				padding: 1px;
				height: 100%;

				width: 100%;
				object-fit: cover;
				vertical-align: bottom;
			}
		}

		.condensed-spacer {
			flex-grow: 11;
		}
	}

	.photo-modal {
		position: fixed;
		top: 0;
		left: 0;
		bottom: 0;
		right: 0;
		background-color: #118ab2 !important;
		display: flex;
		flex-direction: column;
		z-index: 1000;

		.swiper {
			flex: 1;
			width: 100%;
			.img-responsive {
				max-height: calc(100vh - 60px);
			}
		}

		.modal-close {
			position: fixed;
			color: white;
			font-size: 1.5rem;
			line-height: 1.5rem;
			right: 0.8rem;
			top: 0.8rem;
			z-index: 1000;
		}

		.photo-modal-toolbar {
			background: white;
			padding: 0.6rem;
			border-radius: 0.3rem 0.3rem 0 0;
			.label {
				margin: 0.2rem 0;
			}
		}
	}

	.photo-overlay {
		background-color: #118ab2 !important;
	}
	.no-padding {
		padding: 0 !important;
	}
</style>
