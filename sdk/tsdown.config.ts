import { defineConfig } from 'tsdown';

export default defineConfig({
    entry: ['src/index.ts'],
    format: ['esm', 'cjs'],
    fixedExtension: true,
    dts: true,
    // One output file per source module instead of one bundle. With `"sideEffects": false` in
    // package.json, consumer bundlers then drop whole modules the app never imports, together with
    // the dependencies those modules import (a single bundle keeps every bare import it starts with).
    unbundle: true,
    treeshake: {
        // Unused calls to these are dropped from the build output; rolldown otherwise keeps them
        // because it cannot tell they have no side effects.
        manualPureFunctions: ['parseAbi', 'createBase58check'],
    },
});
