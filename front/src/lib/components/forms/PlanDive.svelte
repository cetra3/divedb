<script lang="ts">
	import { client } from '$lib/graphql/client';
	import type { DivePlanInput, DiveScheduleNodeFragment, GasInput } from '$lib/graphql/generated';
	import type { ClientError } from 'graphql-request';

	import FormRow from '../FormRow.svelte';
	import formatGas from '$lib/util/formatGas';
	import { onMount } from 'svelte';

	let errors: string | undefined = $state(undefined);
	let loading = $state(false);

	let depth = $state(18);
	let runtime = $state(30);
	let gflo = $state(50);
	let gfhigh = $state(70);

	let backGas: GasInput = $state({
		litres: 12,
		o2: 21,
		he: 0
	});

	let decoGasses: GasInput[] = $state([]);

	let allGasses: GasInput[] = [];

	let schedule: DiveScheduleNodeFragment | undefined = $state();

	const addDecoGas = () => {
		let newGas = { litres: 12, o2: 21, he: 0 };
		if (allGasses[decoGasses.length]) {
			newGas = allGasses[decoGasses.length];
		}

		decoGasses = [...decoGasses, newGas];
		planDive();
	};

	const removeDecoGas = () => {
		decoGasses.pop();

		decoGasses = [...decoGasses];

		planDive();
	};

	onMount(() => {
		let decoPlan = localStorage.getItem('decoPlan');

		if (decoPlan != null) {
			let existingPlan: DivePlanInput = JSON.parse(decoPlan);
			depth = existingPlan.depth;
			runtime = existingPlan.time;
			gflo = existingPlan.gfLow;
			gfhigh = existingPlan.gfHigh;
			backGas = existingPlan.backGas;
			decoGasses = existingPlan.decoGasses;
			allGasses = existingPlan.decoGasses;
		}

		planDive();
	});

	const onSubmit = (e: Event) => {
		e.preventDefault();
		planDive();
	};

	const planDive = () => {
		let plan: DivePlanInput = {
			gfHigh: gfhigh,
			gfLow: gflo,
			backGas,
			decoGasses,
			depth,
			time: runtime
		};

		for (let i = 0; i < decoGasses.length; i++) {
			allGasses[i] = decoGasses[i];
		}

		errors = undefined;
		loading = true;

		client
			.planDive({ plan })
			.then((mutation) => {
				schedule = mutation.planDive;

				localStorage.setItem('decoPlan', JSON.stringify(plan));
				loading = false;
			})
			.catch((reason: ClientError) => {
				loading = false;
				errors = reason.response.errors?.map((val) => val.message).join();
			});
	};

	$effect(() => {
		console.log(decoGasses, runtime, gfhigh, gflo);
	});
</script>

<div class="columns">
	<div class="column col-6 col-sm-12">
		<form class="form-horizontal" onsubmit={onSubmit}>
			<FormRow name="Depth/Time">
				<div class="columns">
					<div class="column col-6 col-sm-12 gas-row">
						<input
							class="form-input"
							step="any"
							type="number"
							bind:value={depth}
							onchange={planDive}
							placeholder="18.0"
						/>
						<span class="form-input-hint">meters</span>
					</div>
					<div class="column col-6 col-sm-12">
						<input
							class="form-input"
							step="any"
							type="number"
							bind:value={runtime}
							onchange={planDive}
							placeholder="min"
						/>
						<span class="form-input-hint">min</span>
					</div>
				</div>
			</FormRow>
			<FormRow name="Back Gas">
				<div class="columns">
					<div class="column col-4 col-sm-6">
						<input
							class="form-input"
							step="any"
							type="number"
							bind:value={backGas.o2}
							placeholder="O2 %"
							onchange={planDive}
						/>
						<span class="form-input-hint">O2 %</span>
					</div>
					<div class="column col-4 col-sm-6">
						<input
							class="form-input"
							step="any"
							type="number"
							bind:value={backGas.he}
							placeholder="He %"
							onchange={planDive}
						/>
						<span class="form-input-hint">He %</span>
					</div>
				</div>
			</FormRow>
			<FormRow name="Deco Gases">
				{#each decoGasses as gas}
					<div class="columns">
						<div class="column col-4 col-sm-6">
							<input
								class="form-input"
								step="any"
								type="number"
								bind:value={gas.o2}
								placeholder="O2 %"
								onchange={planDive}
							/>
							<span class="form-input-hint">O2 %</span>
						</div>
						<div class="column col-4 col-sm-6">
							<input
								class="form-input"
								step="any"
								type="number"
								bind:value={gas.he}
								placeholder="He %"
								onchange={planDive}
							/>
							<span class="form-input-hint">He %</span>
						</div>
					</div>
				{/each}
				<button class="btn btn-sm gas-row" type="button" onclick={addDecoGas}>Add Gas</button>
				{#if decoGasses.length > 0}
					<button class="btn btn-sm gas-row" type="button" onclick={removeDecoGas}>Remove Gas</button>
				{/if}
			</FormRow>

			<FormRow name="GF Lo/GF Hi">
				<div class="columns">
					<div class="column col-4 col-sm-6">
						<input
							class="form-input"
							type="number"
							step="any"
							bind:value={gflo}
							placeholder="50"
							onchange={planDive}
						/>
						<span class="form-input-hint">GF Low</span>
					</div>
					<div class="column col-4 col-sm-6">
						<input
							class="form-input"
							type="number"
							step="any"
							bind:value={gfhigh}
							onchange={planDive}
							placeholder="70"
						/>
						<span class="form-input-hint">GF High</span>
					</div>
				</div>
			</FormRow>

			<FormRow name="">
				<button class="btn btn-primary" type="submit">Plan Dive</button>
				{#if loading}
					<span class="loading padding-20"></span>
				{/if}
			</FormRow>
		</form>
	</div>

	{#if schedule}
		<div class="column col-6 col-sm-12">
			<div class="card">
				<div class="card-image">
					{@html schedule.smallChart}
				</div>
				<div class="card-header">
					<h4>Dive Plan</h4>
					<strong>Duration:</strong>
					{Math.trunc(schedule.runtime)} min,
					<strong>TTS:</strong>
					{Math.trunc(schedule.tts)} min
				</div>
				<div class="card-body">
					<table>
						<tbody>
							<tr>
								<th align="left" colspan="2">Depth</th>
								<th align="left" class="stage-value">Time</th>
								<th align="left" class="stage-value">Gas</th>
							</tr>
							{#each schedule.stages as stage}
								<tr>
									<td>
										{#if stage.stageType == 'ASCEND'}
											↗️
										{:else if stage.stageType == 'DESCEND'}
											↘️
										{:else if stage.stageType == 'STAY'}
											➡️
										{:else if stage.stageType == 'GAS_CHANGE'}
											🔄
										{/if}
									</td>
									<td class="stage-value">
										{Math.trunc(stage.depth)}m
									</td>
									<td class="stage-value">
										{Math.trunc(stage.time)}min
									</td>
									<td class="stage-value">
										{#if stage.stageType == 'GAS_CHANGE'}
											{formatGas(stage.gas ?? { o2: 21, he: 0 })}
										{/if}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		</div>
	{/if}
</div>

<style lang="scss" global>
   	.gf-row {
		margin-top: 1rem;
	}

	.gas-row {
		margin-bottom: 1rem;
	}

	.stage-type {
		padding-right: 1rem;
	}

	.stage-value {
		padding: 0 0.5rem;
	}
</style>
