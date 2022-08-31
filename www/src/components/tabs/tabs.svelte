<script>
  import { onMount } from 'svelte';

  export let tabs;
  export let onSelect;

  let selectedTab = 0;

  const handleClick = (index) => {
    selectedTab = index;
    onSelect(selectedTab);
  };

  onMount(() => onSelect(selectedTab));
</script>

<div class="tabs is-centered">
  {#each tabs as tab, index}
    <ul>
      <li class:is-active={index === selectedTab} on:click={() => handleClick(index)}>
        <a href="https://" on:click|preventDefault>{ tab }</a>
      </li>
    </ul>
  {/each}
</div>

<style lang="scss">

</style>
