<script lang="ts">
	import type { CreateDive, DiveNodeFragment } from '$lib/graphql/generated';
	import formatMinutes from '$lib/util/formatMinutes';
	import FormRow from '../FormRow.svelte';
	import ChooseSite from './ChooseSite.svelte';
	export let dive: CreateDive | DiveNodeFragment;
	export let onSave: (site: CreateDive) => void;

	const pad2 = (input: number): string => {
		return input.toString().padStart(2, '0');
	};

	const durationRegex = /^([-+]?\d+):([-+]?\d{2})$/;

	let date = new Date(dive.date);

	let dateValue = `${date.getFullYear()}-${pad2(date.getMonth() + 1)}-${pad2(date.getDate())}`;
	let timeValue = `${pad2(date.getHours())}:${pad2(date.getMinutes())}`;

	let duration = formatMinutes(dive.duration);

	$: dateParts = dateValue.split('-');
	$: timeParts = timeValue.split(':');

	$: canSave =
		dive.depth != 0 &&
		dateParts.length == 3 &&
		timeParts.length == 2 &&
		duration.match(durationRegex) != undefined;

	const onSubmit = (e: Event) => {
		e.preventDefault();

		let durationParts = duration.split(':');

		onSave({
			id: dive.id,
			depth: dive.depth,
			duration: +durationParts[0] * 60 + +durationParts[1],
			date: new Date(`${dateValue}T${timeValue}`).toISOString(),
			diveSiteId: dive.diveSiteId
		});
	};
</script>

<div class="columns">
	<div class="column col-12 col-sm-12">
		<form class="form-horizontal" on:submit={onSubmit}>
			<FormRow name="Date">
				<div class="columns">
					<div class="column col-6 col-sm-12">
						<input class="form-input" type="date" bind:value={dateValue} />
					</div>
					<div class="column col-6 col-sm-12">
						<input class="form-input" type="time" bind:value={timeValue} />
					</div>
				</div>
			</FormRow>
			<FormRow name="Dive Site">
				<ChooseSite bind:id={dive.diveSiteId} belowInput={true} />
			</FormRow>
			<FormRow name="Depth (m)">
				<input
					class="form-input"
					step="any"
					type="number"
					placeholder="18.0"
					bind:value={dive.depth}
				/>
			</FormRow>
			<FormRow name="Duration (mm:ss)">
				<input class="form-input" placeholder="mm:ss" bind:value={duration} />
			</FormRow>
			<FormRow name="">
				<button class="btn btn-primary" type="submit" disabled={canSave == false}
					>Submit Dive</button
				>
			</FormRow>
		</form>
	</div>
</div>
