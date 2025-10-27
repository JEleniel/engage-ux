import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import cspDirectives from './svelte.config.csp.js';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	compilerOptions: {
		runes: true,
	},
	kit: {
		adapter: adapter(),
		csp: {
			mode: 'auto',
			directives: cspDirectives,
		},
	},
};

export default config;
