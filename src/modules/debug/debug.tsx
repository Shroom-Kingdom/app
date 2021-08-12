import React, { FC, useContext, useState } from 'react';

import { Button } from '../button';

import { DebugContext } from '.';
import { DebugRow } from './debug-row';

export const Debug: FC = () => {
  const [showDebug, setShowDebug] = useState<boolean>(true);
  const debugState = useContext(DebugContext);
  return (
    <div className="debug">
      <style jsx>{`
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
      `}</style>
      <Button
        onClick={() => {
          setShowDebug(!showDebug);
        }}
      >
        Toggle Debug Info
      </Button>
      {showDebug && (
        <div className="table">
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
      )}
    </div>
  );
};
