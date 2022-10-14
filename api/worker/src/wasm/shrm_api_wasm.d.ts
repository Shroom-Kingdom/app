/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} buf
* @returns {boolean}
*/
export function isCourse(buf: Uint8Array): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly BrotliEncoderCompressMulti: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => number;
  readonly CBrotliDecoderErrorString: (a: number) => number;
  readonly CBrotliDecoderHasMoreOutput: (a: number) => number;
  readonly BrotliDecoderHasMoreOutput: (a: number) => number;
  readonly CBrotliDecoderTakeOutput: (a: number, b: number) => number;
  readonly CBrotliDecoderIsUsed: (a: number) => number;
  readonly CBrotliDecoderIsFinished: (a: number) => number;
  readonly BrotliDecoderIsFinished: (a: number) => number;
  readonly CBrotliDecoderGetErrorCode: (a: number) => number;
  readonly CBrotliDecoderGetErrorString: (a: number) => number;
  readonly BrotliDecoderGetErrorString: (a: number) => number;
  readonly BrotliEncoderCreateInstance: (a: number, b: number, c: number) => number;
  readonly BrotliEncoderSetParameter: (a: number, b: number, c: number) => number;
  readonly BrotliEncoderDestroyInstance: (a: number) => void;
  readonly BrotliEncoderIsFinished: (a: number) => number;
  readonly BrotliEncoderHasMoreOutput: (a: number) => number;
  readonly BrotliEncoderSetCustomDictionary: (a: number, b: number, c: number) => void;
  readonly BrotliEncoderTakeOutput: (a: number, b: number) => number;
  readonly BrotliEncoderVersion: () => number;
  readonly BrotliEncoderMaxCompressedSize: (a: number) => number;
  readonly BrotliEncoderCompress: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly BrotliEncoderCompressStreaming: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly BrotliEncoderCompressStream: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly BrotliEncoderMallocU8: (a: number, b: number) => number;
  readonly BrotliEncoderFreeU8: (a: number, b: number, c: number) => void;
  readonly BrotliEncoderMallocUsize: (a: number, b: number) => number;
  readonly BrotliEncoderFreeUsize: (a: number, b: number, c: number) => void;
  readonly BroccoliCreateInstance: (a: number) => void;
  readonly BroccoliCreateInstanceWithWindowSize: (a: number, b: number) => void;
  readonly BroccoliDestroyInstance: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number, u: number, v: number, w: number, x: number, y: number, z: number, a1: number, b1: number, c1: number, d1: number, e1: number, f1: number, g1: number, h1: number, i1: number, j1: number, k1: number, l1: number, m1: number, n1: number, o1: number, p1: number, q1: number, r1: number, s1: number, t1: number, u1: number, v1: number, w1: number, x1: number, y1: number, z1: number, a2: number, b2: number, c2: number, d2: number, e2: number, f2: number, g2: number, h2: number, i2: number, j2: number, k2: number, l2: number, m2: number, n2: number, o2: number, p2: number, q2: number, r2: number, s2: number, t2: number, u2: number, v2: number, w2: number, x2: number, y2: number, z2: number, a3: number, b3: number, c3: number, d3: number, e3: number, f3: number, g3: number, h3: number, i3: number, j3: number, k3: number, l3: number, m3: number, n3: number, o3: number, p3: number, q3: number, r3: number, s3: number, t3: number, u3: number, v3: number, w3: number, x3: number, y3: number, z3: number, a4: number, b4: number, c4: number, d4: number, e4: number, f4: number, g4: number, h4: number, i4: number, j4: number, k4: number, l4: number, m4: number, n4: number, o4: number, p4: number, q4: number) => void;
  readonly BroccoliNewBrotliFile: (a: number) => void;
  readonly BroccoliConcatStream: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly BroccoliConcatStreaming: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly BroccoliConcatFinish: (a: number, b: number, c: number) => number;
  readonly BroccoliConcatFinished: (a: number, b: number, c: number) => number;
  readonly BrotliEncoderMaxCompressedSizeMulti: (a: number, b: number) => number;
  readonly BrotliEncoderCreateWorkPool: (a: number, b: number, c: number, d: number) => number;
  readonly BrotliEncoderDestroyWorkPool: (a: number) => void;
  readonly BrotliEncoderCompressWorkPool: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => number;
  readonly BrotliDecoderCreateInstance: (a: number, b: number, c: number) => number;
  readonly BrotliDecoderSetParameter: (a: number, b: number, c: number) => void;
  readonly BrotliDecoderDecompressPrealloc: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => void;
  readonly BrotliDecoderDecompressWithReturnInfo: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly BrotliDecoderDecompress: (a: number, b: number, c: number, d: number) => number;
  readonly BrotliDecoderDecompressStream: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly BrotliDecoderDecompressStreaming: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly BrotliDecoderMallocU8: (a: number, b: number) => number;
  readonly BrotliDecoderFreeU8: (a: number, b: number, c: number) => void;
  readonly BrotliDecoderMallocUsize: (a: number, b: number) => number;
  readonly BrotliDecoderFreeUsize: (a: number, b: number, c: number) => void;
  readonly BrotliDecoderDestroyInstance: (a: number) => void;
  readonly BrotliDecoderTakeOutput: (a: number, b: number) => number;
  readonly BrotliDecoderIsUsed: (a: number) => number;
  readonly BrotliDecoderGetErrorCode: (a: number) => number;
  readonly BrotliDecoderErrorString: (a: number) => number;
  readonly BrotliDecoderVersion: () => number;
  readonly isCourse: (a: number, b: number) => number;
  readonly __wbindgen_export_0: (a: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
