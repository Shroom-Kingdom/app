<script lang="ts">
  import { get } from 'svelte/store';

  import type { NearRegister, Account } from '../../../../common-types';
  import Form from '../../components/form/Form.svelte';
  import Input from '../../components/form/Input.svelte';

  import { account$, walletId$, afterRegister$, isRegistered$ } from '.';

  let loading = false;

  interface RegisterForm {
    username: string;
  }

  function handleSubmit(
    _event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement },
    value: RegisterForm
  ) {
    const walletId = $walletId$;
    if (!walletId) return;
    const registerNear: NearRegister = {
      username: value.username,
      walletId
    };
    register(registerNear);
  }

  async function register(registerNear: NearRegister) {
    loading = true;
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
    const account = await res.json<Account>();
    account$.set(account);

    const tryLogin = async () => {
      const isRegistered = await get(isRegistered$);
      if (isRegistered || get(afterRegister$) >= 100) {
        loading = false;
        return;
      }
      afterRegister$.update(prev => prev + 1);
      setTimeout(tryLogin, 2_000);
    };
    tryLogin();
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
  <Form submitLabel="Register" submitForm="{handleSubmit}" loading="{loading}">
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
