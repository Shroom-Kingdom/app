<script lang="ts">
  export let inline = false;

  export let width: number | null = null;
  export let height: number | null = null;

  let style = '';
  $: if (width && height) {
    let min: number = Math.min(width, height) - 18;
    style = `width: ${min}px; height: ${min}px`;
  } else {
    style = '';
  }
</script>

<div class="{`wrapper${inline ? ' inline' : ''}`}">
  <div class="progress-spinner" style="{style}"></div>
</div>

<style>
  .progress-spinner {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    border: 7px solid transparent;
    border-top-color: rgba(0, 0, 0, 0.6);
    animation: rotate 800ms linear infinite;
  }

  .wrapper {
    display: flex;
    width: 100%;
    max-height: 100%;
    align-items: center;
    justify-content: center;
  }

  .wrapper:not(.inline) {
    position: fixed;
    top: 0;
    left: 0;
    height: 100%;
    z-index: 100;
    background-color: rgba(0, 0, 0, 0.3);
  }

  @keyframes rotate {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
