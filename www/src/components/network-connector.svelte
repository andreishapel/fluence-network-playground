<script>
  import { Fluence } from '@fluencelabs/fluence';
  import { krasnodar } from '@fluencelabs/fluence-network-environment';

  let isDropdownOpen = false;
  const peersIds = Object
    .values(krasnodar)
    .map(({ peerId }) => peerId);

  const handleClick = () => {
    isDropdownOpen = !isDropdownOpen;
  }
</script>

<section>
  <p class="is-size-5">
    Connect to the network
  </p>
  <section class="dropdown" class:is-active={isDropdownOpen}>
    <aside class="dropdown-trigger">
      <button class="button is-size-6" on:click={handleClick}>
        <span>Pick a relay</span>
      </button>
    </aside>
    <aside class="dropdown-menu">
      <aside class="dropdown-content">
        {#each peersIds as peerId}
          <p class="dropdown-item">
            {peerId}
          </p>
        {/each}
      </aside>
    </aside>
  </section>
</section>

<style lang="scss">
  .dropdown {
    margin-top: 5px;
  }
</style>
