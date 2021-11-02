<script lang="ts">
  import DebugRow from './DebugRow.svelte';
  import Button from '../button/Button.svelte';
  import {
    stepTime,
    collisionDetectionTime,
    broadPhaseTime,
    narrowPhaseTime,
    islandConstructionTime,
    solverTime,
    velocityAssemblyTime,
    velocityResolutionTime,
    velocityUpdateTime,
    positionAssemblyTime,
    positionResolutionTime,
    ccdTime,
    numSubsteps,
    toiComputationTime,
    ccdBroadPhaseTime,
    ccdNarrowPhaseTime,
    ccdSolverTime
  } from '.';
  import { writable } from 'svelte/store';

  let showDebug = false;

  let test = writable(12);

  $: test.subscribe(asd => {
    console.log('SUBSCRIBE', asd)
  })

  function increment () {
    console.log('click');
    test.update(a => a + 1);
    // test += 1;
  }
</script>


<div class="debug">
  <Button
    on:click={() => {
      showDebug= !showDebug;
    }}
  >
    Toggle Debug Info
  </Button>
  <button on:click={increment}>
    increment
  </button>
  <div>Value is {$test}</div>
  {#if showDebug}
    <div class="table">
      <DebugRow title="Total" value={$stepTime} />
      <DebugRow
        title="Collision detection"
        value={$collisionDetectionTime}
      />
      <DebugRow title="|_ Broad-phase" value={$broadPhaseTime} />
      <DebugRow
        title="|_ Narrow-phase"
        value={$narrowPhaseTime}
      />
      <DebugRow
        title="Island computation"
        value={$islandConstructionTime}
      />
      <DebugRow title="Solver" value={$solverTime} />
      <DebugRow
        title="|_ Velocity assembly"
        value={$velocityAssemblyTime}
      />
      <DebugRow
        title="|_ Velocity resolution"
        value={$velocityResolutionTime}
      />
      <DebugRow
        title="|_ Velocity integration"
        value={$velocityUpdateTime}
      />
      <DebugRow
        title="|_ Position assembly"
        value={$positionAssemblyTime}
      />
      <DebugRow
        title="|_ Position resolution"
        value={$positionResolutionTime}
      />
      <DebugRow title="CCD" value={$ccdTime} />
      <DebugRow
        title="|_ # of substeps"
        value={$numSubsteps}
        isInt
      />
      <DebugRow
        title="|_ TOI computation"
        value={$toiComputationTime}
      />
      <DebugRow
        title="|_ Broad-phase"
        value={$ccdBroadPhaseTime}
      />
      <DebugRow
        title="|_ Narrow-phase"
        value={$ccdNarrowPhaseTime}
      />
      <DebugRow title="|_ Solver" value={$ccdSolverTime} />
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
