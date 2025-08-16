<script lang="ts">
	interface Props {
		name: string;
		error?: boolean | undefined;
		children?: import('svelte').Snippet;
	}

	let { name, error = undefined, children }: Props = $props();
</script>

<div class="form-group" class:has-error={error}>
	<div class="col-3 col-sm-12">
		<label for={undefined} class="form-label">{name}</label>
	</div>
	<div class="col-9 col-sm-12">
		{@render children?.()}
	</div>
</div>
