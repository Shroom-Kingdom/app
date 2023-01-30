<script lang="ts">
  import ProgressSpinner from '../progress/ProgressSpinner.svelte';

  export let primary = true;
  export let size: 'small' | 'medium' | 'large' = 'medium';
  export let loading = false;
  export let disabled = false;
  export let href: string | null = null;
  export let type = 'button' as
    | 'button'
    | 'submit'
    | 'reset'
    | null
    | undefined;

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

{#if href}
  <a href="{href}">
    <button
      on:click
      class="{classes.join(' ')}"
      class:loading="{loading}"
      class:href="{href}"
      style="{style}"
      type="{type}"
      disabled="{disabled}"
      bind:clientWidth="{clientWidth}"
      bind:clientHeight="{clientHeight}"
    >
      {#if loading}
        <ProgressSpinner
          inline
          width="{clientWidth - 4}"
          height="{clientHeight - 4}"
        />
      {:else}
        <slot />
      {/if}
    </button>
  </a>
{:else}
  <button
    on:click
    class="{classes.join(' ')}"
    class:loading="{loading}"
    style="{style}"
    type="{type}"
    disabled="{disabled}"
    bind:clientWidth="{clientWidth}"
    bind:clientHeight="{clientHeight}"
  >
    {#if loading}
      <ProgressSpinner
        inline
        width="{clientWidth - 4}"
        height="{clientHeight - 4}"
      />
    {:else}
      <slot />
    {/if}
  </button>
{/if}

<style lang="scss" module="scoped">
  a {
    margin: var(--margin, 0);
  }

  .button.href {
    margin: 0;
  }

  .button:not(.href) {
    margin: var(--margin, 0);
  }

  .button {
    &.primary {
      background-color: #1678c2;
      &:hover:not(:disabled) {
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
      &:not(.loading) {
        padding: var(--padding, 0.3rem 0.6rem);
      }
      font-size: var(--font-size, 0.8rem);
    }
    &.medium {
      &:not(.loading) {
        padding: var(--padding, 0.5rem 0.9rem);
      }
      font-size: var(--font-size, 1rem);
    }
    &.large {
      &:not(.loading) {
        padding: var(--padding, 0.7rem 1.2rem);
      }
      font-size: var(--font-size, 1.2rem);
    }

    &:disabled {
      background-color: rgb(46, 44, 44);
      cursor: default;
    }

    display: var(--display, inline-block);
    flex-direction: var(--flex-direction, unset);
    width: var(--width, auto);
    min-width: var(--min-width, auto);
    max-width: var(--max-width, auto);
    flex: var(--flex, unset);
    align-items: center;
    justify-content: center;
    color: #fff;
    text-shadow: none;
    box-shadow: 0 0 0 0 rgba(34, 36, 38, 0.15) inset;
    cursor: pointer;
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
