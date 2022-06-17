<script>
  import { Link } from 'svelte-routing';

  export let title;
  export let href;
  export let pathname;
</script>

<li class:is-active={pathname === href}>
  <Link to={href}>
    {title}
  </Link>
</li>
