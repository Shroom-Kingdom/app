export {};

declare global {
  interface Window {
    shrm: { main: (data: Uint8Array) => void };
  }
}
