import React, { FC, useContext, useState } from 'react';

import { Button } from '../button';

import { AssetContext } from '.';

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
    <>
      <Button
        onClick={() => {
          if (assetInput) {
            assetInput.click();
          }
        }}
        loading={loading}
      >
        Select
      </Button>
      <input
        type="file"
        accept=".tar"
        ref={ref => (assetInput = ref)}
        style={{ display: 'none' }}
        onChange={handleSelect}
      />
    </>
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
