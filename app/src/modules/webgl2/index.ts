export function checkWebGL2(): boolean {
  const gl = document.createElement('canvas').getContext('webgl2');
  return !!gl;
}
