import React, { FC, useContext, useState } from 'react';

import { DebugContext } from '.';

export const App: FC = () => {
  const [showDebug, setShowDebug] = useState<boolean>(true);
  const debugState = useContext(DebugContext);
  return (
    <div
      style={{
        position: 'relative'
      }}
    >
      <button
        onClick={() => {
          setShowDebug(!showDebug);
        }}
      >
        Toggle Debug Info
      </button>
      {showDebug && (
        <div style={{ display: 'flex', flexDirection: 'column' }}>
          <div>Total: {debugState.stepTime}</div>
          <div>Collision detection: {debugState.collisionDetectionTime}</div>
          <div>Broad-phase: {debugState.broadPhaseTime}</div>
          <div>Narrow-phase: {debugState.narrowPhaseTime}</div>
          <div>Island computation: {debugState.islandConstructionTime}</div>
        </div>
      )}
    </div>
  );
};
