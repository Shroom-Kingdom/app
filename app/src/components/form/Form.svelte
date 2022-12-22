<script lang="ts">
  import Button from '../button/Button.svelte';

  export let submitLabel: string;
  export let submitForm: (
    event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    value: any
  ) => unknown;

  function submit(
    event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement }
  ) {
    event.preventDefault();

    const form = event.target as HTMLFormElement;
    const formData = new FormData(form);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    submitForm(event, Object.fromEntries(formData as any));
  }
</script>

<form on:submit="{submit}">
  <slot />

  <Button type="submit">{submitLabel}</Button>
</form>
