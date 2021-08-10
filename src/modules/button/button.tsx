import React, { ButtonHTMLAttributes, FC } from 'react';

import { ProgressSpinner } from '../progress/progress-spinner';

type TButton<T = unknown> = FC<{ loading?: boolean } & ButtonHTMLAttributes<T>>;

export const Button: TButton = ({ children, loading, ...props }) => (
  <button className="button" {...props}>
    <style jsx>{`
      .button {
      }
    `}</style>
    {loading ? <ProgressSpinner inline={false} /> : children}
  </button>
);
