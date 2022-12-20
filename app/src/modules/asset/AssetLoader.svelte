<script lang="ts">
  import Alert from '../alert/Alert.svelte';
  import Button from '../button/Button.svelte';

  import { assets } from '.';

  let loading = false;
  let assetInput: HTMLInputElement | undefined;

  const handleSelect = async (event: Event) => {
    if (!(event.target instanceof HTMLInputElement) || !event.target?.files)
      return;
    const file = event.target.files[0];
    if (!file) return;
    loading = true;
    try {
      const newData = await parseFile(file);
      assets.set(newData);
    } catch (err) {
      console.error(err);
      loading = false;
    }
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
</script>

<div class="asset-loader">
  <Alert className="margin-0-1-2-1">
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
  <Button
    on:click="{() => {
      if (assetInput) {
        assetInput.click();
      }
    }}"
    primary
    size="large"
    loading="{loading}"
  >
    Select your asset file
  </Button>
  <input
    bind:this="{assetInput}"
    type="file"
    accept=".tar"
    style="display: none;"
    on:change="{handleSelect}"
  />
</div>

<style>
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
  :global(.margin-0-1-2-1) {
    margin: 0 1rem 2rem 1rem;
  }
</style>
