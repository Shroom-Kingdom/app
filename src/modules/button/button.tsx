import React, { ButtonHTMLAttributes, FC } from 'react';
import { match } from 'ts-pattern';

import { ProgressSpinner } from '../progress/progress-spinner';

type TButton<T = unknown> = FC<
  {
    primary?: boolean;
    size?: 'small' | 'medium' | 'large';
    loading?: boolean;
  } & ButtonHTMLAttributes<T>
>;

export const Button: TButton = ({
  children,
  primary = false,
  size = 'medium',
  loading,
  ...props
}) => (
  <button className="button" {...props}>
    <style jsx>{`
      .button {
        color: #fff;
        background-color: ${primary ? '#1678c2' : '#414242'};
        text-shadow: none;
        box-shadow: 0 0 0 0 rgba(34, 36, 38, 0.15) inset;
        cursor: pointer;
        display: inline-block;
        min-height: 1em;
        outline: 0;
        border: none;
        vertical-align: baseline;
        font-family: Lato, 'Helvetica Neue', Arial, Helvetica, sans-serif;
        margin: ${match(size)
          .with('small', () => '0.2rem 0.25rem')
          .with('medium', () => '0.3rem 0.4rem')
          .with('large', () => '0.4rem 0.55rem')
          .exhaustive()};
        padding: ${match(size)
          .with('small', () => '0.3rem 0.6rem')
          .with('medium', () => '0.5rem 0.9rem')
          .with('large', () => '0.7rem 1.2rem')
          .exhaustive()};
        text-transform: none;
        text-shadow: none;
        font-size: ${match(size)
          .with('small', () => '0.8rem')
          .with('medium', () => '1rem')
          .with('large', () => '1.2rem')
          .exhaustive()};
        font-weight: 600;
        line-height: 1em;
        font-style: normal;
        text-align: center;
        text-decoration: none;
        border-radius: 0.3rem;
        transition: opacity 0.1s ease, background-color 0.1s ease,
          color 0.1s ease, box-shadow 0.1s ease, background 0.1s ease,
          -webkit-box-shadow 0.1s ease;
      }
      .button:hover {
        background-color: ${primary ? '#2185d0' : '#252525'};
      }
    `}</style>
    {loading ? <ProgressSpinner inline={false} /> : children}
  </button>
);
