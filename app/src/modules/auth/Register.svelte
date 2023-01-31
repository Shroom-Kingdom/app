<script lang="ts">
  import type { NearRegister, Account } from '../../../../common-types';
  import Form from '../../components/form/Form.svelte';
  import Input from '../../components/form/Input.svelte';

  import { account$, walletId$, isRegistered$, fetchWithAuth } from '.';

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
    try {
      loading = true;
      // eslint-disable-next-line @typescript-eslint/no-empty-function
      isRegistered$.set(new Promise(() => {}));
      const res = await fetchWithAuth(
        'https://shrm-api.shrm.workers.dev/auth/register/near',
        {
          method: 'POST',
          body: JSON.stringify(registerNear)
        }
      );
      if (!res.ok) {
        console.error(await res.text());
        isRegistered$.set(Promise.resolve(false));
        loading = false;
        return;
      }
      const account = await res.json<Account>();
      isRegistered$.set(Promise.resolve(true));
      account$.set(account);
    } catch (err) {
      isRegistered$.set(Promise.resolve(false));
      loading = false;
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
