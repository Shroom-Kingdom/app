<script lang="ts">
  import type {
    BrowserWalletMetadata,
    InjectedWalletMetadata,
    ModuleState,
    Wallet,
    WalletSelector as NearWalletSelector
  } from '@near-wallet-selector/core';

  import Install from '../../assets/Install.svelte';
  import shardDog from '../../assets/sharddog.webp';
  import Button from '../../components/button/Button.svelte';
  import Modal from '../layout/Modal.svelte';

  import { WALLETS, selector$ } from '.';

  // needed to fix types into discriminated union for Svelte template
  interface BaseWallet {
    id: string;
  }
  interface BrowserWallet extends BaseWallet {
    type: 'browser';
    metadata: BrowserWalletMetadata;
  }
  interface InjectedWallet extends BaseWallet {
    type: 'injected';
    metadata: InjectedWalletMetadata;
  }

  type UnionModuleState = BrowserWallet | InjectedWallet;

  $: mapModules($selector$);

  let mods: UnionModuleState[] = [];
  function mapModules(selector: NearWalletSelector | null) {
    if (!selector) return;
    mods = selector.store.getState().modules.map((mod): UnionModuleState => {
      switch (mod.type) {
        case 'injected':
          return {
            ...mod,
            type: 'injected',
            metadata: mod.metadata as InjectedWalletMetadata
          };
        case 'browser':
          return { ...mod, type: 'browser' };
        default:
          throw new Error('unimplemented');
      }
    });
  }

  async function handleWalletClick(unionMod: UnionModuleState) {
    const mod = unionMod as ModuleState<Wallet>;
    const wallet = await mod.wallet();
    const contractId = 'near-chan-v14.shrm.testnet';

    switch (wallet.type) {
      case 'browser':
      case 'injected':
        await wallet.signIn({
          contractId
        });
        break;
      default:
        throw new Error('unimplemented');
    }
  }

  function isMobile() {
    let check = false;
    (function (a) {
      if (
        /(android|bb\d+|meego).+mobile|avantgo|bada\/|blackberry|blazer|compal|elaine|fennec|hiptop|iemobile|ip(hone|od)|iris|kindle|lge |maemo|midp|mmp|mobile.+firefox|netfront|opera m(ob|in)i|palm( os)?|phone|p(ixi|re)\/|plucker|pocket|psp|series(4|6)0|symbian|treo|up\.(browser|link)|vodafone|wap|windows ce|xda|xiino/i.test(
          a
        ) ||
        /1207|6310|6590|3gso|4thp|50[1-6]i|770s|802s|a wa|abac|ac(er|oo|s-)|ai(ko|rn)|al(av|ca|co)|amoi|an(ex|ny|yw)|aptu|ar(ch|go)|as(te|us)|attw|au(di|-m|r |s )|avan|be(ck|ll|nq)|bi(lb|rd)|bl(ac|az)|br(e|v)w|bumb|bw-(n|u)|c55\/|capi|ccwa|cdm-|cell|chtm|cldc|cmd-|co(mp|nd)|craw|da(it|ll|ng)|dbte|dc-s|devi|dica|dmob|do(c|p)o|ds(12|-d)|el(49|ai)|em(l2|ul)|er(ic|k0)|esl8|ez([4-7]0|os|wa|ze)|fetc|fly(-|_)|g1 u|g560|gene|gf-5|g-mo|go(\.w|od)|gr(ad|un)|haie|hcit|hd-(m|p|t)|hei-|hi(pt|ta)|hp( i|ip)|hs-c|ht(c(-| |_|a|g|p|s|t)|tp)|hu(aw|tc)|i-(20|go|ma)|i230|iac( |-|\/)|ibro|idea|ig01|ikom|im1k|inno|ipaq|iris|ja(t|v)a|jbro|jemu|jigs|kddi|keji|kgt( |\/)|klon|kpt |kwc-|kyo(c|k)|le(no|xi)|lg( g|\/(k|l|u)|50|54|-[a-w])|libw|lynx|m1-w|m3ga|m50\/|ma(te|ui|xo)|mc(01|21|ca)|m-cr|me(rc|ri)|mi(o8|oa|ts)|mmef|mo(01|02|bi|de|do|t(-| |o|v)|zz)|mt(50|p1|v )|mwbp|mywa|n10[0-2]|n20[2-3]|n30(0|2)|n50(0|2|5)|n7(0(0|1)|10)|ne((c|m)-|on|tf|wf|wg|wt)|nok(6|i)|nzph|o2im|op(ti|wv)|oran|owg1|p800|pan(a|d|t)|pdxg|pg(13|-([1-8]|c))|phil|pire|pl(ay|uc)|pn-2|po(ck|rt|se)|prox|psio|pt-g|qa-a|qc(07|12|21|32|60|-[2-7]|i-)|qtek|r380|r600|raks|rim9|ro(ve|zo)|s55\/|sa(ge|ma|mm|ms|ny|va)|sc(01|h-|oo|p-)|sdk\/|se(c(-|0|1)|47|mc|nd|ri)|sgh-|shar|sie(-|m)|sk-0|sl(45|id)|sm(al|ar|b3|it|t5)|so(ft|ny)|sp(01|h-|v-|v )|sy(01|mb)|t2(18|50)|t6(00|10|18)|ta(gt|lk)|tcl-|tdg-|tel(i|m)|tim-|t-mo|to(pl|sh)|ts(70|m-|m3|m5)|tx-9|up(\.b|g1|si)|utst|v400|v750|veri|vi(rg|te)|vk(40|5[0-3]|-v)|vm40|voda|vulc|vx(52|53|60|61|70|80|81|83|85|98)|w3c(-| )|webc|whit|wi(g |nc|nw)|wmlb|wonu|x700|yas-|your|zeto|zte-/i.test(
          a.substr(0, 4)
        )
      )
        check = true;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    })(navigator.userAgent || navigator.vendor || (window as any).opera);
    return check;
  }
</script>

<Modal header="Select Wallet">
  {#if $selector$}
    <div class="wallets">
      {#each mods as mod}
        <Button
          --margin="0.6rem"
          --padding="0"
          --flex="1 0 calc(50% - 2 * 0.6rem)"
          --min-width="200px"
          size="medium"
          disabled="{!mod.metadata.available || WALLETS[mod.id].disabled}"
          on:click="{() => handleWalletClick(mod)}"
        >
          <div class="wallet">
            <img src="{mod.metadata.iconUrl}" alt="{mod.metadata.name}" />
            <div class="wallet-name">
              <span>{mod.metadata.name}</span>
              {#if mod.metadata.description != null}
                <span class="url">{new URL(WALLETS[mod.id].url).hostname}</span>
              {/if}
            </div>
            {#if mod.type === 'injected' && !isMobile()}
              {#if WALLETS[mod.id].extensionUrl != null}
                <a
                  href="{WALLETS[mod.id].extensionUrl}"
                  target="_blank"
                  rel="noreferrer"
                  class="download"
                  on:click|stopPropagation
                >
                  <Install />
                </a>
              {:else if mod.metadata.downloadUrl != null}
                <a
                  href="{mod.metadata.downloadUrl}"
                  target="_blank"
                  rel="noreferrer"
                  class="download"
                  on:click|stopPropagation
                >
                  <Install />
                </a>
              {/if}
            {/if}
          </div>
        </Button>
      {/each}
      {#if mods.length % 2 === 1}
        <div style="flex: 1 0 calc(50% - 2 * 0.6rem); margin: 0.6rem;"></div>
      {/if}
    </div>
  {/if}

  <h4>Don't have a wallet?</h4>
  <div class="footer">
    <span>The easiest way to get a named Near wallet is via ShardDog.</span>
    <Button
      size="large"
      --display="flex"
      href="https://testnet.shard.dog/go?url={window.location.href}"
    >
      <img src="{shardDog}" alt="sharddog icon" />Get Wallet
    </Button>
  </div>
  <h4>Why ShardDog?</h4>
  <div class="footer">
    <ul>
      <li>
        no need to reserve an account name. Instantly claim your preferred
        account name
      </li>
      <li>get some free $NEAR to pay for gas fees</li>
      <li>
        if you're coming from another blockchain like Ethereum, you don't need
        to go through a CEX to fund your named wallet
      </li>
    </ul>
  </div>
</Modal>

<style lang="scss">
  .wallets {
    display: flex;
    flex-wrap: wrap;
  }

  h4 {
    padding-top: 1.6rem;
    margin-top: 1.2rem;
    margin-bottom: 0.6rem;
    border-top: 1px solid lightgray;
  }

  .wallet {
    display: flex;
    margin: 0.6rem;
    align-items: center;
    --img-size: 1.6rem;

    img {
      min-width: var(--img-size);
      min-height: var(--img-size);
      max-width: var(--img-size);
      max-height: var(--img-size);
      margin-right: 0.5rem;
    }

    .download {
      margin-left: auto;
      z-index: 100;
      padding: 2px;

      &:hover {
        background-color: rgba(13, 1, 46, 0.4);
        border-radius: 4px;
      }
    }
  }

  .wallet-name {
    display: flex;
    flex-direction: column;
    align-items: flex-start;

    .url {
      margin-top: 0.2rem;
      flex: 1 0 auto;
      font-size: 0.7rem;
      color: rgba(255, 255, 255, 0.7);
      height: 0.8rem;
    }
  }

  .footer {
    max-width: 100%;
    flex: 1 0 auto;
    display: flex;
    flex-direction: column;

    :global(a) {
      margin: 1rem auto;
    }

    img {
      height: 60px;
      margin-right: 0.8rem;
    }
  }
</style>
