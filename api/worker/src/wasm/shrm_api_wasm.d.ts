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
  readonly BrotliDecoderHasMoreOutput: (a: number) => number;
  readonly BrotliDecoderTakeOutput: (a: number, b: number) => number;
  readonly BrotliDecoderIsUsed: (a: number) => number;
  readonly BrotliDecoderIsFinished: (a: number) => number;
  readonly BrotliDecoderGetErrorCode: (a: number) => number;
  readonly BrotliDecoderGetErrorString: (a: number) => number;
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
