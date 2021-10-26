import { Writable, writable } from 'svelte/store';

export interface DebugState {
  stepTime: number;
  collisionDetectionTime: number;
  broadPhaseTime: number;
  narrowPhaseTime: number;
  islandConstructionTime: number;
  solverTime: number;
  velocityAssemblyTime: number;
  velocityResolutionTime: number;
  velocityUpdateTime: number;
  positionAssemblyTime: number;
  positionResolutionTime: number;
  ccdTime: number;
  numSubsteps: number;
  toiComputationTime: number;
  ccdBroadPhaseTime: number;
  ccdNarrowPhaseTime: number;
  ccdSolverTime: number;
}

export interface DebugContext {
  state: Writable<DebugState>;
}

export const initialDebugState: DebugContext = {
  state: writable<DebugState>({
    stepTime: 0,
    collisionDetectionTime: 0,
    broadPhaseTime: 0,
    narrowPhaseTime: 0,
    islandConstructionTime: 0,
    solverTime: 0,
    velocityAssemblyTime: 0,
    velocityResolutionTime: 0,
    velocityUpdateTime: 0,
    positionAssemblyTime: 0,
    positionResolutionTime: 0,
    ccdTime: 0,
    numSubsteps: 0,
    toiComputationTime: 0,
    ccdBroadPhaseTime: 0,
    ccdNarrowPhaseTime: 0,
    ccdSolverTime: 0
  })
};

export const debugKey = {};
