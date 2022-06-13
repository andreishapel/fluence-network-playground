<script>
  import { onMount, onDestroy } from 'svelte';
  import { krasnodar } from '@fluencelabs/fluence-network-environment';
  import NetworkStore from '../../store/network.store';
  import NetworkConnectorConnectButton from './network-connector-connect-button.svelte';
  import NetworkConnectorDropdown from './network-connector-dropdown';

  const statusPooling = NetworkStore.useStatusPooling();
  const peers = Object.values(krasnodar);

  let selectedPeer;

  $: networkStatus = NetworkStore.store;

  const handleDropdownItemRender = (peer) => {
    const { peerId } = peer;
    return peerId;
  };

  const selectPeer = (peer) => {
    selectedPeer = peer;
  };

  const connect = () => {
    if (selectedPeer) {
      NetworkStore.connectToPeer(selectedPeer);
    }
  };

  const disconnect = () => {
    if (selectedPeer) {
      NetworkStore.disconnect();
    }
  };

  onMount(() => NetworkStore.useStatusPooling());
  onDestroy(() => statusPooling());
</script>

<section class="network-connector">
  <header class="header is-size-4">
    Connect to the network
  </header>
  <p class="is-size-5 connection-status" class:is-active={$networkStatus.isConnected}>
    Connection: {($networkStatus.isConnected) ? 'Connected' : 'Disconnected'}
  </p>
  {#if $networkStatus.isInitialized && $networkStatus.isConnected}
    <aside>
      <span class="network-status-property">
        Peer id:
      </span>
      <span>
        {$networkStatus.peerId}
      </span>
    </aside>
    <aside>
      <span class="network-status-property">
        Relay peer id:
      </span>
      <span>
        {$networkStatus.relayPeerId}
      </span>
    </aside>
  {/if}
  <section class="container">
    <NetworkConnectorDropdown peers={peers} onSelect={selectPeer} onDropdownItemRender={handleDropdownItemRender} />
    <NetworkConnectorConnectButton networkStatus={$networkStatus} handleConnect={connect} handleDisconnect={disconnect} />
  </section>
</section>

<style lang="scss">
  @import "src/styles/variables";

  .network-connector {
    margin: 75px auto;
    width: 675px;
  }

  .header {
    text-align: center;
  }

  .connection-status {
    color: $color-red;

    &.is-active {
      color: $color-green;
    }
  }

  .network-status-property {
    font-weight: bold;
  }

  .container {
    margin-top: 5px;
  }
</style>
