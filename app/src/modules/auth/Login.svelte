<script lang="ts">
  import { Sha256 } from '@aws-crypto/sha256-browser';
  import {
    type WalletSelector as NearWalletSelector,
    setupWalletSelector
  } from '@near-wallet-selector/core';
  import { setupMeteorWallet } from '@near-wallet-selector/meteor-wallet';
  import { setupNearWallet } from '@near-wallet-selector/near-wallet';
  import { setupSender } from '@near-wallet-selector/sender';
  import { KeyPairEd25519 } from 'near-api-js/lib/utils';
  import { type SvelteComponentTyped, onMount } from 'svelte';
  import { bind } from 'svelte-simple-modal';

  import type { Account } from '../../../../common-types';
  import Button from '../../components/button/Button.svelte';
  import { ModalSize, modal$, modalSize$ } from '../layout/modal';

  import {
    afterRegister$,
    isRegistered$,
    account$,
    accessToken$,
    selector$,
    walletId$
  } from '.';
  import WalletSelector from './WalletSelector.svelte';

  let isSignedIn: boolean | null = null;

  $: setupWallet($selector$);
  $: signTransaction($walletId$, $afterRegister$);

  onMount(async () => {
    const selector = await setupWalletSelector({
      network: 'testnet',
      modules: [setupNearWallet(), setupSender(), setupMeteorWallet()]
    });
    selector$.set(selector);
  });

  async function showWalletSelector() {
    if (!$selector$) return;

    modalSize$.set(ModalSize.Medium);
    // FIXME types
    modal$.set(bind(WalletSelector as unknown as SvelteComponentTyped, {}));
  }

  async function signOut() {
    if (!$selector$) return;
    const wallet = await $selector$.wallet();
    wallet.signOut();
    isSignedIn = false;
  }

  async function setupWallet(selector: NearWalletSelector | null) {
    if (!selector) return;
    isSignedIn = selector.isSignedIn();
    if (!isSignedIn) {
      showWalletSelector();
    } else {
      walletId$.set(
        selector.store.getState().accounts.find(({ active }) => active)
          ?.accountId ?? null
      );
    }
  }

  async function signTransaction(
    walletId: string | null,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _afterRegister: number
  ) {
    if (!walletId) return;
    try {
      // eslint-disable-next-line @typescript-eslint/no-empty-function
      isRegistered$.set(new Promise(() => {}));
      const accessToken = await createAccessToken(walletId);

      const res = await fetch('https://shrm-api.shrm.workers.dev/auth/login', {
        method: 'POST',
        body: accessToken
      });
      if (!res.ok) {
        console.error(await res.text());
        isRegistered$.set(Promise.resolve(false));
        return;
      }

      if (res.status === 204) {
        isRegistered$.set(Promise.resolve(false));
        return;
      }
      isRegistered$.set(Promise.resolve(true));
      const user = await res.json<Account>();
      account$.set(user);
      accessToken$.set(accessToken);
    } catch (err) {
      isRegistered$.set(Promise.resolve(false));
    }
  }

  async function createAccessToken(walletId: string): Promise<string> {
    // TODO this uses `near-api-js` as long as wallet selector cannot sign messages.
    // Until then only Near Wallet is supported
    const keyStore = window.localStorage.getItem(
      `near-api-js:keystore:${walletId}:testnet`
    );
    if (!keyStore) {
      throw new Error(`No keystore found for walletId ${walletId}`);
    }
    const keyPair = new KeyPairEd25519(keyStore.replace('ed25519:', ''));

    const tokenMessage = btoa(
      JSON.stringify({
        walletId,
        iat: new Date().getTime()
      })
    );
    const message = new TextEncoder().encode(tokenMessage);

    const hash = new Sha256();
    hash.update(message);
    const result = await hash.digest();
    const signature = keyPair.sign(result);
    return (
      tokenMessage +
      '.' +
      btoa(signature.signature.toString()) +
      '.' +
      keyPair.publicKey.toString()
    );
  }
</script>

<div class="login">
  {#if selector$}
    {#if isSignedIn === true}
      {$walletId$}
      <Button on:click="{() => signOut()}" primary size="small">Logout</Button>
    {:else}
      <Button on:click="{() => showWalletSelector()}" primary size="medium">
        Login with Near
      </Button>
    {/if}
  {/if}
</div>

<style>
  .login {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>
