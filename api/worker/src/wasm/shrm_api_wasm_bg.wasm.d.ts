/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function BrotliDecoderCreateInstance(a: number, b: number, c: number): number;
export function BrotliDecoderSetParameter(a: number, b: number, c: number): void;
export function BrotliDecoderDecompressPrealloc(a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number): void;
export function BrotliDecoderDecompressWithReturnInfo(a: number, b: number, c: number, d: number, e: number): void;
export function BrotliDecoderDecompress(a: number, b: number, c: number, d: number): number;
export function BrotliDecoderDecompressStream(a: number, b: number, c: number, d: number, e: number, f: number): number;
export function BrotliDecoderDecompressStreaming(a: number, b: number, c: number, d: number, e: number): number;
export function BrotliDecoderMallocU8(a: number, b: number): number;
export function BrotliDecoderFreeU8(a: number, b: number, c: number): void;
export function BrotliDecoderMallocUsize(a: number, b: number): number;
export function BrotliDecoderFreeUsize(a: number, b: number, c: number): void;
export function BrotliDecoderDestroyInstance(a: number): void;
export function BrotliDecoderHasMoreOutput(a: number): number;
export function BrotliDecoderTakeOutput(a: number, b: number): number;
export function BrotliDecoderIsUsed(a: number): number;
export function BrotliDecoderIsFinished(a: number): number;
export function BrotliDecoderGetErrorCode(a: number): number;
export function BrotliDecoderGetErrorString(a: number): number;
export function BrotliDecoderErrorString(a: number): number;
export function BrotliDecoderVersion(): number;
export function isCourse(a: number, b: number): number;
export function __wbindgen_export_0(a: number): number;
