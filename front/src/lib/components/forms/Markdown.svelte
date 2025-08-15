<script lang="ts">
	import { Editor, Viewer } from 'bytemd';

	export let value: string;

	function handleChange(e: any) {
		value = e.detail.value;
	}
</script>

<Editor mode={'tab'} {value} on:change={handleChange} />

<style global lang="scss">
	@import 'bytemd/dist/index.css';
	.bytemd {
		border: 0.05rem solid #bcc3ce;
		border-radius: 0.3rem;
	}

	.bytemd-toolbar {
		border-top-right-radius: 0.3rem;
		border-top-left-radius: 0.3rem;
	}

	.bytemd-toolbar-tab-active {
		color: #0e5e7c;
	}

	.bytemd,
	.bytemd-toolbar {
		font-family:
			'Asap',
			-apple-system,
			system-ui,
			BlinkMacSystemFont,
			'Segoe UI',
			Roboto !important;
	}

	.bytemd-editor .CodeMirror {
		.CodeMirror-lines {
			padding: 0.25rem 0.4rem !important;
		}
		pre.CodeMirror-line,
		pre.CodeMirror-line-like {
			font-family:
				'Asap',
				-apple-system,
				system-ui,
				BlinkMacSystemFont,
				'Segoe UI',
				Roboto !important;
			font-size: 0.8rem;
			color: #24292e;

			padding: 0 !important;
		}
	}

	.bytemd-preview .markdown-body {
		padding: 0.25rem 0.4rem !important;
	}

	.bytemd-toolbar-right [bytemd-tippy-path='0'] {
		display: none;
	}

	.bytemd-toolbar-right [bytemd-tippy-path='4'] {
		display: none;
	}

	.bytemd-toolbar-right [bytemd-tippy-path='5'] {
		display: none;
	}
</style>
