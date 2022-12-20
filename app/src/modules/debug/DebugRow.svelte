<script lang="ts">
  export let title: string;
  export let value: number;
  export let isInt = false;
</script>

<div class="row">
  <div>
    {isInt
      ? value
      : new Intl.NumberFormat('en-EN', {
          minimumFractionDigits: 2,
          maximumFractionDigits: 2
        }).format(value)}
  </div>
  <div>{title}:</div>
</div>

<style>
  .row {
    display: flex;
    flex-direction: row-reverse;
  }
  .row > :nth-child(2) {
    flex: 1 0 180px;
    margin-right: 1rem;
  }
  .row > :nth-child(1) {
    flex: 0 0 36px;
    margin: 0 0.6rem;
  }
</style>
