import React, { FC, useContext, useState } from 'react';
import css from 'styled-jsx/css';

import { Button } from '../button';
import { Alert } from '../alert';

import { AssetContext } from '.';

const { className: alertClass, styles } = css.resolve`
  div {
    margin: 0 1rem 2rem 1rem;
  }
`;

export const AssetLoader: FC = () => {
  const [loading, setLoading] = useState(false);
  const { setAssetState } = useContext(AssetContext);
  let assetInput: HTMLInputElement | null = null;

  const handleSelect = async (event: React.ChangeEvent<HTMLInputElement>) => {
    if (!event.target.files) return;
    const file = event.target.files[0];
    if (!file) return;
    setLoading(true);
    try {
      const data = await parseFile(file);
      setAssetState({ data });
    } catch (err) {
      console.error(err);
      setLoading(false);
    }
  };

  return (
    <div className="asset-loader">
      <style jsx>{`
        .asset-loader {
          max-width: calc(100% - 4rem);
          overflow: hidden;
          display: flex;
          flex-direction: column;
          justify-content: space-around;
          width: 100%;
          height: 100%;
          align-items: center;
          justify-content: center;
          padding: 2rem;
        }
      `}</style>
      <Alert className={alertClass}>
        If you don&apos;t yet have an asset file, please go to our Github and
        download the{' '}
        <a
          href="https://github.com/Shroom-Kingdom/asset-extractor/releases"
          target="_blank"
          rel="noreferrer noopener"
        >
          latest release from the asset extractor
        </a>{' '}
        and follow its instructions.
      </Alert>
      {styles}
      <Button
        onClick={() => {
          if (assetInput) {
            assetInput.click();
          }
        }}
        primary
        size="large"
        loading={loading}
      >
        Select your asset file
      </Button>
      <input
        type="file"
        accept=".tar"
        ref={ref => (assetInput = ref)}
        style={{ display: 'none' }}
        onChange={handleSelect}
      />
    </div>
  );
};

async function parseFile(file: File): Promise<Uint8Array> {
  const buffer = await readFile(file);
  return new Uint8Array(buffer);
}

async function readFile(file: File): Promise<ArrayBuffer> {
  return new Promise(resolve => {
    const reader = new FileReader();
    reader.addEventListener('loadend', () => {
      resolve(reader.result as ArrayBuffer);
    });
    reader.readAsArrayBuffer(file);
  });
}
