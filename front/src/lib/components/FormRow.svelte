<script lang="ts">
	export let name: string;
	export let error: boolean | undefined = undefined;
</script>

<div class="form-group" class:has-error={error}>
	<div class="col-3 col-sm-12">
		<label for={undefined} class="form-label">{name}</label>
	</div>
	<div class="col-9 col-sm-12">
		<slot />
	</div>
</div>
