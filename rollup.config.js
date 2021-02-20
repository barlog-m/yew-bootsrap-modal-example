import { nodeResolve } from '@rollup/plugin-node-resolve';

export default {
    input: "app.js",
    output: {
        file: "pkg/bundle.js",
        format: "iife",
    },
    plugins: [nodeResolve()]
};
