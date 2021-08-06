import React, { FC } from 'react';

import type { Browser } from '.';

export function checkWebGL2(): boolean {
  const gl = document.createElement('canvas').getContext('webgl2');
  return !!gl;
}

export const WebGL2FixHint: FC<{ browser: Browser }> = ({ browser }) => {
  const defaultMessage = (
    <div>
      Please visit{' '}
      <a href="https://get.webgl.org/webgl2/" target="_blank" rel="noreferrer">
        this website
      </a>{' '}
      to get instructions how to enable WebGL2.
    </div>
  );
  if (!browser) {
    return defaultMessage;
  }
  switch (browser.os) {
    case 'iOS':
      return (
        <div>
          You can enable WebGL2 for Safari under &apos;Settings&apos; &gt;
          &apos;Experimental Features&apos; &gt; &apos;WebGL 2.0&apos;
        </div>
      );
    default:
      return defaultMessage;
  }
  return <div></div>;
};
