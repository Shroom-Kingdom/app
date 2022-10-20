<script lang="ts">
  import { keyStores, WalletConnection, connect, ConnectConfig, Near } from '@tarnadas/near-api-js'
  
  import Button from '../button/Button.svelte';

  let near: Near | null = null;
  let wallet: WalletConnection | null = null;

  const nearConfig: ConnectConfig = {
    networkId: 'testnet',
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
  };

  async function setupWallet() {
    near = await connect(nearConfig);
    wallet = new WalletConnection(near, null);
  }
  setupWallet();

  async function signTransaction() {
    if (!wallet) return;
    const accessToken = await createAccessToken(wallet);
    console.log('accessToken', accessToken)

    const res = await fetch('https://shrm-api.shrm.workers.dev/auth/login', {
      method: 'POST',
      body: accessToken
    });
    if (!res.ok) {
      console.error(await res.text());
      return;
    }
    const asd = await res.json();
    console.log('asd', asd)
  }

  async function createAccessToken(wallet: WalletConnection): Promise<string> {
    const accountId = wallet.getAccountId();
    console.log('accountId', accountId)
    if (!accountId) {
      await wallet.requestSignIn({
        contractId: 'near-chan-v14.shrm.testnet'
      });
      return '';
    }
    const tokenMessage = btoa(JSON.stringify({ accountId: accountId, iat: new Date().getTime() }));
    try {
      const signature = await wallet.account()
          .connection.signer
          .signMessage(new TextEncoder().encode(tokenMessage), accountId, nearConfig.networkId);
      return tokenMessage + '.' + btoa(String.fromCharCode(...signature.signature));
    } catch (err) {
      wallet.signOut();
      return '';
    }
}
</script>

<div class="login">
  {#if wallet && near}
    <Button
      on:click={() => signTransaction()}
      primary
      size="large"
    >
    Login with Near
    </Button>
  {/if}
</div>

<style>
  .login {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>
