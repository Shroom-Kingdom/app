<script lang="ts">
  import { colorWarning } from '../color';
  import Icon from '../icon/Icon.svelte';

  export let className = '';

  const style = `border: 2px solid ${colorWarning};`;
</script>

<div class="alert {className || ''}" style="{style}">
  <Icon type="warning" class="flex-content-36" />
  <div class="content">
    <slot />
  </div>
</div>

<style lang="scss" module="scoped">
  .alert {
    display: flex;
    align-items: center;
    border-radius: 0.8rem;
    padding: 0.4rem 0.6rem;
    max-width: 36rem;
  }
  .content {
    flex: 1 1 auto;
    max-width: 100%;
    text-align: center;
    padding: 0.4rem 0.6rem;
  }
  :global(.flex-content-36) {
    flex: 0 0 36px;
  }
</style>
