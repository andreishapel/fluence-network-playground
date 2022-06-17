<script>
  import get from 'lodash/get';
  import { onMount } from 'svelte';
  import { Router, Route } from 'svelte-routing';
  import { globalHistory } from 'svelte-routing/src/history';
  import NetworkStore from '@store/network.store';
  import NavigationLink from '@components/navigation/navigation-link.svelte';
  import Chat from '@components/chat/chat.svelte';

  const links = [{
    title: 'Chat',
    href: '/chat',
  }, {
    title: 'Hashing',
    href: '/hash',
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
      <Route path="chat" component={Chat} />
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
