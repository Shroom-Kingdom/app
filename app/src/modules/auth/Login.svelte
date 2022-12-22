<script lang="ts">
  import {
    keyStores,
    WalletConnection,
    connect,
    ConnectConfig,
    Near
  } from '@tarnadas/near-api-js';

  import type { Account } from '../../../../common-types';
  import Button from '../../components/button/Button.svelte';

  import { isRegistered$, account$, walletId$ } from '.';

  let near: Near | null = null;
  let wallet: WalletConnection | null = null;

  const nearConfig: ConnectConfig = {
    networkId: 'testnet',
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org'
  };

  login();

  async function login() {
    const walletId = await setupWallet();
    console.log('walletId', walletId);
    await signTransaction(walletId);
  }

  async function setupWallet(): Promise<string> {
    near = await connect(nearConfig);
    wallet = new WalletConnection(near, null);
    await wallet.isSignedInAsync();
    const accountId = wallet.getAccountId();
    walletId$.set(wallet.getAccountId());
    return accountId;
  }

  async function signTransaction(walletId: string | null) {
    if (!wallet || !walletId) return;
    const accessToken = await createAccessToken(wallet, walletId);

    const res = await fetch('https://shrm-api.shrm.workers.dev/auth/login', {
      method: 'POST',
      body: accessToken
    });
    if (!res.ok) {
      console.error(await res.text());
      return;
    }

    if (res.status === 204) {
      isRegistered$.set(false);
      return;
    }
    isRegistered$.set(true);
    const user = await res.json<Account>();
    account$.set(user);
  }

  async function createAccessToken(
    wallet: WalletConnection,
    walletId: string
  ): Promise<string> {
    const tokenMessage = btoa(
      JSON.stringify({ accountId: walletId, iat: new Date().getTime() })
    );
    try {
      const signature = await wallet
        .account()
        .connection.signer.signMessage(
          new TextEncoder().encode(tokenMessage),
          walletId,
          nearConfig.networkId
        );
      return (
        tokenMessage + '.' + btoa(String.fromCharCode(...signature.signature))
      );
    } catch (err) {
      wallet.signOut();
      return '';
    }
  }

  async function handleLogin() {
    if (!wallet) return;
    await wallet.requestSignIn({
      contractId: 'near-chan-v14.shrm.testnet'
    });
  }

  async function handleLogout() {
    if (!wallet) return;
    wallet.signOut();
    walletId$.set(null);
  }
</script>

<div class="login">
  {#if wallet && near}
    {#if $walletId$}
      {$walletId$}
      <Button on:click="{() => handleLogout()}" primary size="small">
        Logout
      </Button>
    {:else}
      <Button on:click="{() => handleLogin()}" primary size="medium">
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
