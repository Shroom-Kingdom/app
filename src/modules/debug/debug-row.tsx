import React, { FC } from 'react';

const numberFormatter = new Intl.NumberFormat('en-EN', {
  minimumFractionDigits: 2,
  maximumFractionDigits: 2
});

export const DebugRow: FC<{ title: string; value: number; isInt?: boolean }> =
  ({ title, value, isInt = false }) => (
    <div className="row">
      <style jsx>{`
        .row {
          display: flex;
          flex-direction: row-reverse;
        }
        .row > :nth-child(2) {
          flex: 1 0 180px;
          margin-right: 1rem;
        }
        .row > :nth-child(1) {
          flex: 0 0 36px;
          margin: 0 0.6rem;
        }
      `}</style>
      <div>{isInt ? value : numberFormatter.format(value)}</div>
      <div>{title}:</div>
    </div>
  );
