import adapter from '@sveltejs/adapter-cloudflare';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	vitePlugin: {
		// set to true for defaults or customize with object
		inspector: {
			toggleKeyCombo: 'control-shift',
			showToggleButton: 'always',
			toggleButtonPos: 'bottom-right'
		}
	},

	preprocess: [vitePreprocess()],

	kit: {
		alias: {
			$components: './src/lib/components'
		},
		// @ ts-ignore
		adapter: adapter({
			// See below for an explanation of these options
			routes: {
				include: ['/*'],
				exclude: ['<all>']
			},
			platformProxy: {
				persist: './your-custom-path'
			}
		})
	}
};

export default config;
