import React, { FC } from 'react';
import { match } from 'ts-pattern';

import { colorWarning } from '../color';

import Warning from './warning.svg';

export const Icon: FC<{
  type: 'warning';
  className?: string;
}> = ({ type, className }) => (
  <div className={`icon ${className ?? ''}`}>
    <style jsx>{`
      .icon {
        height: 36px;
        width: 36px;
        color: ${match(type)
          .with('warning', () => colorWarning)
          .exhaustive()};
      }
    `}</style>
    {match(type)
      .with('warning', () => <Warning />)
      .exhaustive()}
  </div>
);
