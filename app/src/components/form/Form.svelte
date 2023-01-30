<script lang="ts">
  import Button from '../button/Button.svelte';

  export let submitLabel: string;
  export let submitForm: (
    event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    value: any
  ) => unknown;
  export let loading = false;

  function submit(
    event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement }
  ) {
    const form = event.target as HTMLFormElement;
    if (!form.checkValidity()) return;
    const formData = new FormData(form);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    submitForm(event, Object.fromEntries(formData as any));
  }
</script>

<form on:submit|preventDefault="{submit}">
  <slot />

  <Button type="submit" loading="{loading}">{submitLabel}</Button>
</form>
