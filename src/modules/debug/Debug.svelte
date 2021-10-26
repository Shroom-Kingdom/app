<script lang="ts">
  import { getContext } from 'svelte';
  import DebugRow from './DebugRow.svelte';
  import Button from '../button/Button.svelte';
  import { debugKey, DebugContext, DebugState, initialDebugState } from '.';

  let showDebug = false;
  let debugState: DebugState;
	const { state } = getContext<DebugContext>(debugKey);
    state.subscribe(state => {
		if (!state) return;
		debugState = state;
	});
</script>


<div class="debug">
  <Button
    on:click={() => {
      showDebug= !showDebug;
    }}
  >
    Toggle Debug Info
  </Button>
  {#if showDebug}
    <div class="table">
      <DebugRow title="Total" value={debugState.stepTime} />
      <DebugRow
        title="Collision detection"
        value={debugState.collisionDetectionTime}
      />
      <DebugRow title="|_ Broad-phase" value={debugState.broadPhaseTime} />
      <DebugRow
        title="|_ Narrow-phase"
        value={debugState.narrowPhaseTime}
      />
      <DebugRow
        title="Island computation"
        value={debugState.islandConstructionTime}
      />
      <DebugRow title="Solver" value={debugState.solverTime} />
      <DebugRow
        title="|_ Velocity assembly"
        value={debugState.velocityAssemblyTime}
      />
      <DebugRow
        title="|_ Velocity resolution"
        value={debugState.velocityResolutionTime}
      />
      <DebugRow
        title="|_ Velocity integration"
        value={debugState.velocityUpdateTime}
      />
      <DebugRow
        title="|_ Position assembly"
        value={debugState.positionAssemblyTime}
      />
      <DebugRow
        title="|_ Position resolution"
        value={debugState.positionResolutionTime}
      />
      <DebugRow title="CCD" value={debugState.ccdTime} />
      <DebugRow
        title="|_ # of substeps"
        value={debugState.numSubsteps}
        isInt
      />
      <DebugRow
        title="|_ TOI computation"
        value={debugState.toiComputationTime}
      />
      <DebugRow
        title="|_ Broad-phase"
        value={debugState.ccdBroadPhaseTime}
      />
      <DebugRow
        title="|_ Narrow-phase"
        value={debugState.ccdNarrowPhaseTime}
      />
      <DebugRow title="|_ Solver" value={debugState.ccdSolverTime} />
    </div>
  {/if}
</div>

<style>
  .debug {
    position: absolute;
    top: 0;
    right: 0;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }
  .table {
    display: flex;
    flex-direction: column;
  }
</style>
