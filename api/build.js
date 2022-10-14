/* eslint-disable @typescript-eslint/no-var-requires */
const { build } = require('esbuild');

build({
  entryPoints: ['src/index.ts'],
  bundle: true,
  outfile: 'dist/index.mjs',
  format: 'esm',
  minify: true,
  logLevel: 'info'
});
