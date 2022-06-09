import adapter from '@sveltejs/adapter-auto';

export default {
	kit: {
		adapter: adapter(),
		vite: () => ({
			build: {
				target: ['es2020'],
			},
		}),
	},
};
