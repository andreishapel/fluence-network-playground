<script>
  import { clickOutside } from '@helpers/DOM';
  import NetworkConnectorDropdownItem from '@components/network-connector/network-connector-dropdown-item';

  export let peers;
  export let onSelect;
  export let onDropdownItemRender;

  let isDropdownOpen = false;
  let value;

  const handleOpen = () => isDropdownOpen = !isDropdownOpen;
  const closeDropdown = () => isDropdownOpen = false;

  const handleSelect = (peer) => {
    const { peerId } = peer;

    value = peerId;
    onSelect(peer);
    closeDropdown();
  };
</script>

<section class="dropdown" class:is-active={isDropdownOpen}>
  <aside class="dropdown-trigger">
    <button class="button is-size-6" on:click={handleOpen}>
        <span class="title">
          { value || 'Pick a relay' }
        </span>
    </button>
  </aside>
  <aside class="dropdown-menu" use:clickOutside on:clickOutside={closeDropdown}>
    <aside class="dropdown-content">
      {#each peers as peer}
        <NetworkConnectorDropdownItem peer={peer} onSelect={handleSelect} onDropdownItemRender={onDropdownItemRender} />
      {/each}
    </aside>
  </aside>
</section>

<style lang="scss">
  .button {
    width: 485px;
    font-size: 14px;
  }

  .title {
    font-size: 14px;
  }
</style>
