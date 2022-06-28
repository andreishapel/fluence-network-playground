<script>
  import get from 'lodash/get';
  import { onMount } from 'svelte';
  import { Router, Route } from 'svelte-routing';
  import { globalHistory } from 'svelte-routing/src/history';
  import NetworkStore from '@store/network.store';
  import NavigationLink from '@components/navigation/navigation-link.svelte';
  import BitcoinLedger from '@components/bitcoin-ledger/bitcoin-ledger.svelte';
  import Chat from '@components/chat/chat.svelte';
  import Crypto from '@components/crypto/crypto.svelte';
  import CryptoPrice from '@components/crypto-price/crypto-price.svelte';
  import Database from '@components/database/database.svelte';
  import IPFS from '@components/ipfs/ipfs.svelte';

  const links = [{
    title: 'Chat',
    href: '/chat',
  }, {
    title: 'Cryptography',
    href: '/crypto',
  }, {
    title: 'Crypto Price',
    href: '/crypto-price',
  }, {
    title: 'Database',
    href: '/database',
  }, {
    title: 'Bitcoin Ledger',
    href: '/bitcoin-ledger',
  }, {
    title: 'IPFS Integration',
    href: '/ipfs',
  }];

  $: isConnectionActive = NetworkStore.isConnectionActive();
  let currentPath = window.location.pathname;

  onMount(() => {
    globalHistory.listen((params) => {
      currentPath = get(params, ['location', 'pathname']);
    });
  });
</script>

<Router>
  <nav class="breadcrumb">
    <ul class="list">
      {#each links as {href, title}}
        <NavigationLink href={href} title={title} pathname={currentPath} />
      {/each}
    </ul>
  </nav>
  <aside>
    {#if $isConnectionActive}
      <Route path="bitcoin-ledger" component={BitcoinLedger}></Route>
      <Route path="chat" component={Chat} />
      <Route path="crypto" component={Crypto} />
      <Route path="crypto-price" component={CryptoPrice}></Route>
      <Route path="database" component={Database}></Route>
      <Route path="ipfs" component={IPFS}></Route>
    {:else}
      <p class="error">Please connect to the network first</p>
    {/if}
  </aside>
</Router>

<style lang="scss">
  @import "src/styles/variables";

  .list {
    justify-content: center;
  }

  .error {
    color: $color-red;
    font-size: 18px;
    text-align: center;
  }
</style>
