import React, { FC } from 'react';
import css from 'styled-jsx/css';

import { colorWarning } from '../color';
import { Icon } from '../icon';

const { className: iconClass, styles } = css.resolve`
  div {
    flex: 0 0 36px;
  }
`;

export const Alert: FC<{ className?: string }> = ({ children, className }) => (
  <div className={`alert ${className ?? ''}`}>
    <style jsx>{`
      .alert {
        display: flex;
        align-items: center;
        border: 2px solid ${colorWarning};
        border-radius: 0.8rem;
        padding: 0.4rem 0.6rem;
        max-width: 36rem;
      }
      .content {
        flex: 1 1 auto;
        max-width: 100%;
        text-align: center;
        padding: 0.4rem 0.6rem;
      }
    `}</style>
    <Icon type="warning" className={iconClass} />
    {styles}
    <div className="content">{children}</div>
  </div>
);
