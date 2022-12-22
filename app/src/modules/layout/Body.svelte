<script lang="ts">
  import AssetLoader from '../asset/AssetLoader.svelte';
  import { isRegistered$ } from '../auth';
  import Register from '../auth/Register.svelte';
</script>

{#if $isRegistered$}
  <AssetLoader />
{:else}
  <Register />
{/if}
