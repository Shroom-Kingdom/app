<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { Modal } from 'svelte-simple-modal';

  import { assets } from './modules/asset';
  import Body from './modules/layout/Body.svelte';
  import Header from './modules/layout/Header.svelte';
  import { modal$ } from './modules/layout/modal';
  import { ScreenSize, screenSize$ } from './modules/layout/screen-size';

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

  let resizeObserver: ResizeObserver;
  onMount(() => {
    const resizeObserver = new ResizeObserver(entries => {
      for (const entry of entries) {
        const { inlineSize } = entry.contentBoxSize[0];
        if (inlineSize <= ScreenSize.Phone) {
          screenSize$.set(ScreenSize.Phone);
        } else if (inlineSize <= ScreenSize.Mobile) {
          screenSize$.set(ScreenSize.Mobile);
        } else if (inlineSize <= ScreenSize.Laptop) {
          screenSize$.set(ScreenSize.Laptop);
        } else if (inlineSize <= ScreenSize.DesktopLg) {
          screenSize$.set(ScreenSize.DesktopLg);
        } else {
          screenSize$.set(ScreenSize.DesktopXl);
        }
      }
    });

    resizeObserver.observe(window.document.body);
  });
  onDestroy(() => {
    if (!resizeObserver) return;
    resizeObserver.unobserve(window.document.body);
  });
</script>

<Modal
  show="{$modal$}"
  styleWindow="{{
    width: 'var(--modal-width)',
    maxWidth: '100vw',
    maxHeight: 'var(--modal-max-height)',
    margin: 'auto',
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'stretch',
    backgroundColor: '#281c4a',
    border: '2px solid var(--bright-border)',
    boxShadow: '0px 0px 15px #d295ba'
  }}"
  styleWindowWrap="{{
    margin: '0'
  }}"
  styleCloseButton="{{
    cursor: 'pointer',
    borderRadius: '25%'
  }}"
  styleContent="{{
    maxHeight: '100%'
  }}"
/>

{#if assetData == null}
  <div class="app">
    <Header />
    <Body />
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
