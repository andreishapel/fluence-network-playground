<script>
  import 'normalize.css';
  import 'bulma/css/bulma.css';
  import NetworkConnector from '@components/network-connector/network-connector.svelte';
  import Navigation from '@components/navigation/navigation.svelte';
</script>

<main class="root container is-widescreen">
  <NetworkConnector />
  <Navigation />
</main>
