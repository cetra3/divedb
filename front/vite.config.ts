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
		plugins: [sveltekit()]
	};
});
