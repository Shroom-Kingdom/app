<script lang="ts">
  export let name: string;
  export let label: string;
</script>

<div class="input">
  <label for="{name}">{label}</label>
  <input type="text" name="{name}" id="{name}" />
</div>

<style lang="scss">
  .input {
    display: flex;
    flex-direction: column;
  }
</style>
