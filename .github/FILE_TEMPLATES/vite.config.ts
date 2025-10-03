import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import dotenv from 'dotenv';

dotenv.config();

const mode = process.env.NODE_ENV?.toLowerCase() ?? 'production;';
const isProd = mode === 'production';

export default defineConfig({
	mode: mode,
	build: {
		minify: isProd,
		cssMinify: isProd,
	},
	plugins: [tailwindcss(), sveltekit()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}'],
	},
});
