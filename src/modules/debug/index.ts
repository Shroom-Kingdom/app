import { writable } from 'svelte/store';

export const stepTime = writable(0);
export const collisionDetectionTime = writable(0);
export const broadPhaseTime = writable(0);
export const narrowPhaseTime = writable(0);
export const islandConstructionTime = writable(0);
export const solverTime = writable(0);
export const velocityAssemblyTime = writable(0);
export const velocityResolutionTime = writable(0);
export const velocityUpdateTime = writable(0);
export const positionAssemblyTime = writable(0);
export const positionResolutionTime = writable(0);
export const ccdTime = writable(0);
export const numSubsteps = writable(0);
export const toiComputationTime = writable(0);
export const ccdBroadPhaseTime = writable(0);
export const ccdNarrowPhaseTime = writable(0);
export const ccdSolverTime = writable(0);
