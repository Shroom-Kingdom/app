<script lang="ts">
  import type { HtmlInputEvent } from '.';

  export let name: string;
  export let label: string;

  export let validate: ((target: HTMLInputElement) => void) | null = null;

  let touched = false;
  let valid = true;

  function onInput(event: HtmlInputEvent) {
    const target = event.target as HTMLInputElement;
    if (validate) validate(target);
    valid = target.checkValidity();
  }

  function onBlur() {
    touched = true;
  }
</script>

<div class="input">
  <label for="{name}">{label}</label>
  <input
    type="text"
    name="{name}"
    id="{name}"
    on:input="{onInput}"
    on:blur="{onBlur}"
    class:touched="{touched}"
    class:valid="{valid}"
    class:invalid="{!valid}"
  />
</div>

<style lang="scss">
  .input {
    display: flex;
    flex-direction: column;

    > * {
      margin: 0.3rem 0;
    }

    input {
      border-radius: 3px;
      border: 1px solid grey;
      padding: 0.2rem;
      font-size: 1.2rem;

      &.touched.invalid {
        border: 1px solid #c00;

        &:focus {
          outline: 1px solid #c00;
        }
      }
    }
  }
</style>
