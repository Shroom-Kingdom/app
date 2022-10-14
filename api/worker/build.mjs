import { build } from 'esbuild';
import { copyFileSync, mkdirSync } from 'fs';
import { dirname, join, resolve } from 'path';
import { fileURLToPath } from 'url';

const root = fileURLToPath(dirname(import.meta.url));
const dist = join(root, 'dist');

let plugin = {
  name: 'copy-wasm-plugin',

  setup(build) {
    let filter = /\.wasm$/;

    build.onResolve({ filter, namespace: 'file' }, args => {
      let src = resolve(args.resolveDir, args.path);
      let dst = resolve(dist, args.path);
      mkdirSync(dirname(dst), { recursive: true });
      copyFileSync(src, dst);
      return null;
    });
  }
};

build({
  entryPoints: ['src/index.ts'],
  bundle: true,
  outfile: 'dist/index.mjs',
  external: ['*.wasm'],
  format: 'esm',
  minify: false,
  logLevel: 'info',
  plugins: [plugin]
});
