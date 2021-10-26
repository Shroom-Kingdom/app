<script lang="ts">
  import { match } from 'ts-pattern';

  import { colorWarning } from '../color';
  
  import Warning from './Warning.svelte';

  export let type: 'warning' = 'warning';

  const color = match(type)
    .with('warning', () => `color: ${colorWarning};`)
    .exhaustive();
</script>

<div class="icon {$$restProps.class || ''}" style={color}>
  {#if type === 'warning'}
    <Warning />
  {/if}
</div>

<style>
  .icon {
    height: 36px;
    width: 36px;
  }
</style>
