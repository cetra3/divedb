<script lang="ts">
	import type { GetPhotosQueryVariables, PhotoSummaryFragment } from '#lib/graphql/generated.js';
	import { client } from '#lib/graphql/client.js';
	import { throttle } from 'lodash-es';
	import { browser } from '$app/env';
	import { onDestroy, type Snippet } from 'svelte';

	import Swiper from 'swiper';
	import type { VirtualData } from 'swiper/types';
	import { Zoom, Navigation, Pagination, Virtual } from 'swiper/modules';

	import 'swiper/css';
	import 'swiper/css/zoom';
	import 'swiper/css/navigation';
	import 'swiper/css/pagination';
	import 'swiper/css/virtual';
	import PhotoLabels from './photos/PhotoLabels.svelte';
	import imageAlt from '#lib/util/imageAlt.js';
	import PhotoSlide from './photos/PhotoSlide.svelte';
	import Photo from './photos/Photo.svelte';

	interface Props {
		query?: GetPhotosQueryVariables;
		condensed?: boolean;
		photos: PhotoSummaryFragment[];
		/** Extra labels in the photo viewer toolbar, given the current photo's id */
		photoLabel?: Snippet<[string]>;
	}

	let { query = undefined, condensed = false, photos: photosProp, photoLabel }: Props = $props();

	// Grows as more photos are loaded, and resets if the parent passes a new list
	let photos = $derived(photosProp);

	let loading = false;
	let atTheEnd = false;

	let curIdx = $state(0);
	let showModal = $state(false);
	let toEdit: PhotoSummaryFragment | undefined = $state(undefined);

	let swiper: Swiper | undefined;

	// The window of slides Swiper wants rendered; Svelte renders them in the template
	let virtualData: VirtualData<PhotoSummaryFragment> | undefined = $state(undefined);

	const refreshSlides = () => {
		if (swiper) {
			swiper.virtual.slides = photos;
			swiper.virtual.update(true);
		}
	};

	const onEditSave = (newPhoto: PhotoSummaryFragment) => {
		photos = photos.map((photo) => (photo.id == newPhoto.id ? newPhoto : photo));
		toEdit = undefined;
		refreshSlides();
	};

	const openPhoto = (photoId: string) => {
		curIdx = Math.max(
			0,
			photos.findIndex((photo) => photo.id == photoId)
		);
		showModal = true;
		document.body.style.overflow = 'hidden';
	};

	const closeModal = () => {
		document.body.style.overflow = 'visible';
		toEdit = undefined;
		showModal = false;
	};

	const morePhotos = () => {
		if (query !== undefined && !loading && !atTheEnd) {
			loading = true;
			client.getPhotos({ ...query, offset: photos.length }).then((val) => {
				atTheEnd = val.photos.length == 0;
				photos = [...photos, ...val.photos];
				loading = false;
				refreshSlides();
			});
		}
	};

	const initSwiper = (el: HTMLElement) => {
		swiper = new Swiper(el, {
			modules: [Zoom, Navigation, Pagination, Virtual],
			zoom: true,
			initialSlide: curIdx,
			navigation: {
				nextEl: el.querySelector<HTMLElement>('.swiper-button-next'),
				prevEl: el.querySelector<HTMLElement>('.swiper-button-prev')
			},
			pagination: {
				el: el.querySelector<HTMLElement>('.swiper-pagination'),
				dynamicBullets: true,
				clickable: true
			},
			virtual: {
				slides: photos,
				renderExternal: (data) => {
					virtualData = data as VirtualData<PhotoSummaryFragment>;
				},
				renderExternalUpdate: false
			},
			on: {
				slideChange: (s) => {
					curIdx = s.activeIndex;
					toEdit = undefined;
				},
				reachEnd: morePhotos
			}
		});

		return {
			destroy: () => {
				swiper?.destroy();
				swiper = undefined;
				virtualData = undefined;
			}
		};
	};

	// Once the slides Swiper asked for are in the DOM, let it measure them
	$effect(() => {
		if (virtualData && swiper && !swiper.destroyed) {
			swiper.updateSlides();
			swiper.updateProgress();
			swiper.updateSlidesClasses();
			swiper.emit('_virtualUpdated');
		}
	});

	onDestroy(() => {
		if (browser) {
			document.body.style.overflow = 'visible';
		}
	});

	const handleScroll = throttle(() => {
		if (!condensed) {
			let scrollTop = window.scrollY;
			let docHeight = document.body.offsetHeight;
			let winHeight = window.innerHeight;
			let scrollPercent = scrollTop / (docHeight - winHeight);

			if (scrollPercent > 0.5) {
				morePhotos();
			}
		}
	}, 300);
</script>

{#if showModal}
	<div class="photo-modal">
		<div class="modal-close">
			<span class="pointer" onclick={closeModal}>✕ </span>
		</div>
		<div
			class="swiper"
			use:initSwiper
			style="--swiper-navigation-color: #FFF;--swiper-pagination-color: #FFF"
		>
			<div class="swiper-wrapper">
				{#if virtualData}
					{#each virtualData.slides as slide, index (virtualData.from + index)}
						<div
							class="swiper-slide"
							data-swiper-slide-index={virtualData.from + index}
							style="left: {virtualData.offset}px"
						>
							<div class="swiper-zoom-container">
								<Photo
									large={true}
									width={slide.width}
									height={slide.height}
									alt={imageAlt(slide)}
									id={slide.id}
								/>
							</div>
						</div>
					{/each}
				{/if}
			</div>
			<div class="swiper-pagination"></div>
			<div class="swiper-button-prev"></div>
			<div class="swiper-button-next"></div>
		</div>
		<div class="photo-modal-toolbar">
			<PhotoLabels photo={photos[curIdx]}>
				<span
					class="pointer label label-secondary"
					onclick={() => {
						if (toEdit != undefined) {
							toEdit = undefined;
						} else {
							toEdit = photos[curIdx];
						}
					}}>Edit</span
				>
				{@render photoLabel?.(photos[curIdx].id)}
			</PhotoLabels>

			{#if toEdit}
				<PhotoSlide {onEditSave} photo={toEdit} />
			{/if}
		</div>
	</div>
{/if}

<svelte:window onscroll={handleScroll} />
{#if !condensed}
	<div class="columns" class:hide-scroll={showModal}>
		{#each photos as photo}
			<div class="column col-6 col-sm-12">
				<div class="card">
					<div class="card-image">
						<Photo
							alt={imageAlt(photo)}
							width={photo.width}
							height={photo.height}
							imgClass="img-responsive pointer"
							id={photo.id}
							onclick={() => openPhoto(photo.id)}
						/>
					</div>

					<div class="card-footer photo-labels no-padding"><PhotoLabels {photo} /></div>
				</div>
			</div>
		{/each}
	</div>
{:else}
	<div class="condensed">
		{#each photos as photo}
			<div class="condensed-photo">
				<Photo onclick={() => openPhoto(photo.id)} imgClass="pointer" id={photo.id} />
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
