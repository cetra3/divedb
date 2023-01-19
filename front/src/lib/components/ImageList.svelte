<script lang="ts">
	import type { GetPhotosQueryVariables, PhotoSummaryFragment } from '$lib/graphql/generated';
	import { client } from '$lib/graphql/client';
	import { throttle } from 'lodash-es';
	export let query: GetPhotosQueryVariables | undefined = undefined;
	export let photos: PhotoSummaryFragment[];

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

	let scrollPercent = 0;
	let loading = false;
	let atTheEnd = false;

	let showModal = false;
	let initiateModal = false;

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
			});
		}
	};

	const handleScroll = throttle(() => {
		let scrollTop = window.scrollY;
		let docHeight = document.body.offsetHeight;
		let winHeight = window.innerHeight;
		scrollPercent = scrollTop / (docHeight - winHeight);

		if (scrollPercent > 0.5) {
			morePhotos();
		}
	}, 300);
</script>

{#if initiateModal}
	<div class={'modal modal-lg no-padding'} class:active={showModal}>
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<span class="modal-overlay photo-overlay" on:click={() => (showModal = false)} />
		<div class="modal-container no-padding photo-popup">
			<div class="modal-header">
				<!-- svelte-ignore a11y-missing-content -->
				<!-- svelte-ignore a11y-invalid-attribute -->
				<a
					href="javascript:void(0)"
					class="btn btn-clear float-right"
					on:click={() => (showModal = false)}
					aria-label="Close"
				/>
			</div>
			<div class="modal-body no-padding">
				<Swiper
					style="--swiper-navigation-color: #118ab2;--swiper-pagination-color: #118ab2"
					zoom
					autoHeight
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
					on:slideChange={() => {
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
									width={slide.width}
									height={slide.height}
									alt={imageAlt(slide)}
									id={slide.id}
								/>
							</div>
							<div class="photo-labels">
								<!-- svelte-ignore a11y-click-events-have-key-events -->
								<span
									class="pointer label label-secondary"
									on:click={() => {
										if (toEdit != undefined) {
											toEdit = undefined;
										} else {
											toEdit = slide;
										}
									}}>Edit</span
								>
								<PhotoLabels photo={slide} />
								<slot name="photo-label" photo={slide.id} />
							</div>
						</SwiperSlide>
					{/each}
				</Swiper>
				{#if toEdit}
					<PhotoSlide {onEditSave} photo={toEdit} />
				{/if}
			</div>
		</div>
	</div>
{/if}

<svelte:window on:scroll={handleScroll} />
<div class="columns">
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
						on:click={() => {
							initiateModal = true;
							photoId = photo.id;
							updateSwiper();
							showModal = true;
						}}
					/>
				</div>
				<div class="card-footer photo-labels">
					<PhotoLabels {photo} />
				</div>
			</div>
		</div>
	{/each}
</div>

<style global lang="scss">
	.photo-overlay {
		background-color: #118ab2 !important;
	}
	.no-padding {
		padding: 0 !important;
	}
</style>
