import adapter from '@sveltejs/adapter-node';
import { sveltePreprocess } from 'svelte-preprocess';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig(({ mode }) => {
	process.env.BACKEND_URL = 'http://localhost:3333';

	return {
		server: {
			proxy: {
				'/api': {
					target: 'http://localhost:3333'
				}
			}
		},
		plugins: [
			sveltekit({
				// Consult https://svelte.dev/docs/kit/integrations
				// for more information about preprocessors
				preprocess: sveltePreprocess(),
				adapter: adapter(),
				compilerOptions: {
					// Applied by the compiler, so it covers svelte-check as well as dev/build
					warningFilter: (warning) =>
						!warning.code.startsWith('a11y') &&
						warning.code !== 'element_invalid_self_closing_tag'
				}
			})
		]
	};
});
