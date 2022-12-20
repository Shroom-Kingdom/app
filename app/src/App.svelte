<script lang="ts">
  import { onMount } from 'svelte';

  import { assets } from './modules/asset';
  import AssetLoader from './modules/asset/AssetLoader.svelte';
  import Login from './modules/login/Login.svelte';

  let canvas: HTMLCanvasElement | null = null;
  document.body.oncontextmenu = e => {
    e.preventDefault();
    return false;
  };

  disableChromePerformanceBloat();

  // https://github.com/bevyengine/bevy/issues/4851
  function disableChromePerformanceBloat() {
    if (performance.clearMeasures) {
      performance.clearMeasures();
    }
    if (performance.clearMarks) {
      performance.clearMarks();
    }
  }

  let assetData: Uint8Array | null = null;
  assets.subscribe(data => {
    if (!data) return;
    assetData = data;

    if (canvas) {
      canvas.style.zIndex = '0';
      canvas.onblur = () => {
        setTimeout(() => {
          canvas?.focus();
        });
      };
      canvas.focus();
      setInterval(() => {
        if (document.activeElement !== canvas) {
          canvas?.focus();
        }
      }, 1000);
    }

    shrm.main(data);
  });

  let shrm: { main: (data: Uint8Array) => void };
  onMount(async () => {
    let loadedWasmModule = false;
    while (!loadedWasmModule) {
      if (window.shrm) {
        loadedWasmModule = true;
        shrm = window.shrm;
      } else {
        await new Promise(resolve => setTimeout(resolve, 100));
      }
    }

    const loadingIndicator = document.querySelector('#loading-indicator');
    if (loadingIndicator) {
      loadingIndicator.remove();
    }
  });
</script>

{#if assetData == null}
  <div class="app">
    <Login />
    <AssetLoader />
  </div>
{/if}
<canvas
  id="canvas"
  bind:this="{canvas}"
  on:contextmenu="{e => {
    e.preventDefault();
    return false;
  }}"></canvas>

<style>
  .app {
    position: relative;
    display: grid;
  }
  #canvas {
    position: absolute;
    top: 0;
    left: 0;
    z-index: -1;
    width: 1280px;
    height: 720px;
  }
</style>
