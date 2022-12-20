<script lang="ts">
  import ProgressSpinner from '../progress/ProgressSpinner.svelte';

  export let primary = true;
  export let size: 'small' | 'medium' | 'large' = 'medium';
  export let loading = false;

  let clientWidth = 0;
  let clientHeight = 0;
  let style = '';

  $: if (loading) {
    style = `min-width: ${clientWidth}px; max-width: ${clientWidth}px; min-height: ${clientHeight}px; max-height: ${clientHeight}px`;
  } else {
    style = '';
  }

  const classes = ['button', size];
  if (primary) {
    classes.push('primary');
  }
</script>

<button
  on:click
  class="{classes.join(' ')}"
  style="{style}"
  bind:clientWidth="{clientWidth}"
  bind:clientHeight="{clientHeight}"
>
  {#if loading}
    <ProgressSpinner inline width="{clientWidth}" height="{clientHeight}" />
  {:else}
    <slot />
  {/if}
</button>

<style lang="scss" module="scoped">
  .button {
    &.primary {
      background-color: #1678c2;
      &:hover {
        background-color: #2185d0;
      }
    }

    &:not(.primary) {
      background-color: #414242;
      &:hover {
        background-color: #252525;
      }
    }

    &.small {
      margin: 0.2rem 0.25rem;
      padding: 0.3rem 0.6rem;
      font-size: 0.8rem;
    }
    &.medium {
      margin: 0.3rem 0.4rem;
      padding: 0.5rem 0.9rem;
      font-size: 1rem;
    }
    &.large {
      margin: 0.4rem 0.55rem;
      padding: 0.7rem 1.2rem;
      font-size: 1.2rem;
    }

    color: #fff;
    text-shadow: none;
    box-shadow: 0 0 0 0 rgba(34, 36, 38, 0.15) inset;
    cursor: pointer;
    display: inline-block;
    min-height: 1em;
    outline: 0;
    border: none;
    vertical-align: baseline;
    font-family: Lato, 'Helvetica Neue', Arial, Helvetica, sans-serif;
    text-transform: none;
    text-shadow: none;
    font-weight: 600;
    line-height: 1em;
    font-style: normal;
    text-align: center;
    text-decoration: none;
    border-radius: 0.3rem;
    transition: opacity 0.1s ease, background-color 0.1s ease, color 0.1s ease,
      box-shadow 0.1s ease, background 0.1s ease, -webkit-box-shadow 0.1s ease;
  }
</style>
