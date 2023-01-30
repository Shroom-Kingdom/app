<script lang="ts">
  import ProgressSpinner from '../../components/progress/ProgressSpinner.svelte';
  import AssetLoader from '../asset/AssetLoader.svelte';
  import { isRegistered$ } from '../auth';
  import Register from '../auth/Register.svelte';
</script>

{#await $isRegistered$}
  <ProgressSpinner />
{:then isRegistered}
  {#if isRegistered}
    <AssetLoader />
  {:else}
    <Register />
  {/if}
{/await}
