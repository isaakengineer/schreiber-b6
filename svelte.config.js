// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { sveltePreprocess } from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: sveltePreprocess({
  		scss: {
	    	prependData: `@import './src/style.scss';`,
     		// renderSync: true,
       		includePaths: [ './src/style']
    	}
	}),
	kit: {
		adapter: adapter(),
	},
};

export default config;
