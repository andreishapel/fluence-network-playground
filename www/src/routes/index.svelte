<script>
  import { Router, Link, Route } from 'svelte-routing';
  import BrowserToBrowser from '../pages/browser-to-browser.svelte';

  export let url = '';
</script>

<Router>
  <Router url="{url}">
    <nav>
      <Link to="browser-to-browser">Browser to Browser</Link>
    </nav>
    <div>
      <Route path="browser-to-browser" component="{BrowserToBrowser}" />
    </div>
  </Router>
</Router>

<style>
  @import 'bulma/css/bulma.css';
</style>
