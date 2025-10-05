import type { Config } from 'tailwindcss';
import defaultTheme from 'tailwindcss/defaultTheme';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		fontFamily: {
			sans: ['Roboto', ...defaultTheme.fontFamily.sans],
			serif: ['Comfortaa', ...defaultTheme.fontFamily.serif],
			mono: ['Source Code Pro', ...defaultTheme.fontFamily.mono],
		},
	},

	plugins: [],
} as Config;
