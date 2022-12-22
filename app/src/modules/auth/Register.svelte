<script lang="ts">
  import type { NearRegister } from '../../../../common-types';
  import Form from '../../components/form/Form.svelte';
  import Input from '../../components/form/Input.svelte';

  import { walletId$ } from '.';

  interface RegisterForm {
    username: string;
  }

  function handleSubmit(
    event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement },
    value: RegisterForm
  ) {
    walletId$.subscribe(walletId => {
      if (!walletId) return;
      const registerNear: NearRegister = {
        username: value.username,
        walletId
      };
      console.log('registerNear', registerNear);
      // register(registerNear);
    });
  }

  async function register(registerNear: NearRegister) {
    const res = await fetch(
      'https://shrm-api.shrm.workers.dev/auth/register/near',
      {
        method: 'POST',
        body: JSON.stringify(registerNear)
      }
    );
    if (!res.ok) {
      console.error(await res.text());
      return;
    }
  }

  function validateUsername(target: HTMLInputElement) {
    if (target.value.length < 3) {
      target.setCustomValidity('Min length is 3');
    } else if (target.value.length > 15) {
      target.setCustomValidity('Max length is 15');
    } else if (!target.value.match(/^[a-zA-Z0-9_]*$/)) {
      target.setCustomValidity('Only alphanumeric characters are allowed');
    } else {
      target.setCustomValidity('');
    }
  }
</script>

<div class="register">
  <Form submitLabel="Register" submitForm="{handleSubmit}">
    <Input
      name="username"
      label="Please register your account by choosing a username:"
      validate="{validateUsername}"
    />
  </Form>
</div>

<style lang="scss">
  .register {
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
