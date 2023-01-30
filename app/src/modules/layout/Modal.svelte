<script lang="ts">
  export let header: string;
</script>

<div class="modal">
  <h3 class="header">{header}</h3>
  <slot />
</div>

<style lang="scss">
  .modal {
    display: flex;
    flex-direction: column;
    color: white;
  }

  .header {
    margin: 0.4rem 0;
    flex: 1 0 100%;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid lightgray;
  }
</style>
