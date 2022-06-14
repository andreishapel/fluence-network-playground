<script>
  import { onMount, onDestroy } from 'svelte';
  import { krasnodar } from '@fluencelabs/fluence-network-environment';
  import NetworkStore from '@store/network.store';
  import NetworkConnectorConnectButton from '@components/network-connector/network-connector-connect-button.svelte';
  import NetworkConnectorDropdown from '@components/network-connector/network-connector-dropdown';

  const statusPooling = NetworkStore.useStatusPooling();
  const peers = Object.values(krasnodar);

  $: networkStatus = NetworkStore.store;
  $: connectionStatus = ($networkStatus.isConnected) ? 'Connected' : 'Disconnected';
  let selectedPeer;

  const handleDropdownItemRender = (peer) => peer.peerId;
  const selectPeer = (peer) => selectedPeer = peer;
  const connect = () => selectedPeer && NetworkStore.connectToPeer(selectedPeer);
  const disconnect = () => selectedPeer && NetworkStore.disconnect();

  onMount(() => NetworkStore.useStatusPooling());
  onDestroy(() => statusPooling());
</script>

<section class="network-connector">
  <header class="header is-size-4">
    Connect to the network
  </header>
  <p class="is-size-5 connection-status" class:is-active={$networkStatus.isConnected}>
    Connection: {connectionStatus}
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

  .abc {}

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
