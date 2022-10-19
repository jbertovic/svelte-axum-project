import resolve from '@rollup/plugin-node-resolve';
import svelte from 'rollup-plugin-svelte';
import css from 'rollup-plugin-css-only';
import commonjs from '@rollup/plugin-commonjs';
import { terser } from 'rollup-plugin-terser';
import livereload from 'rollup-plugin-livereload';

const production = !process.env.ROLLUP_WATCH;

export default {
	input: 'src/main.js',
	output: {
		sourcemap: true,
		format: 'iife',
		name: 'app',
		file: 'public/build/bundle.js'
	},
    plugins: [
        // teaches rollup how to find stuff in node_modules
        resolve({
            browser: true,
            dedupe: ['svelte']
        }),
        // tells rollup what to do when it sees a svelte file
        svelte({
            compilerOptions: {
                // enable run-time checks when not in productions
                dev: !production
            }
        }),
        // we'll extract any component CSS out into
        // a separate file - better for performance
        css({ output: 'bundle.css'}),
        commonjs(),

		// Watch the `public` directory and refresh the
		// browser on changes when not in production
		!production && livereload('public'),

		// If we're building for production (npm run build
		// instead of npm run dev), minify
		production && terser()
    ],
    watch: {
        clearScreen: false
    }
};