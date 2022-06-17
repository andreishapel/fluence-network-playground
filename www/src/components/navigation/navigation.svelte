<script>
  import lodash from 'lodash';
  import { onMount } from 'svelte';
  import { Router, Route } from 'svelte-routing';
  import { globalHistory } from 'svelte-routing/src/history';
  import NavigationLink from '@components/navigation/navigation-link.svelte';
  import PeerToPeer from '@components/peer-to-peer/peer-to-peer.svelte';

  const links = [{
    title: 'Chat',
    href: '/chat',
  }, {
    title: 'Hashing',
    href: 'hash',
  }];

  let currentPath = window.location.pathname;

  onMount(() => {
    globalHistory.listen((params) => {
      currentPath = lodash.get(params, ['location', 'pathname']);
    });
  });
</script>

<Router>
  <nav class="breadcrumb">
    <ul class="list">
      {#each links as link}
        <NavigationLink href={link.href} title={link.title} pathname={currentPath} />
      {/each}
    </ul>
  </nav>
  <aside>
    <Route path="chat" component={PeerToPeer} />
  </aside>
</Router>

<style lang="scss">
  .list {
    justify-content: center;
  }
</style>
