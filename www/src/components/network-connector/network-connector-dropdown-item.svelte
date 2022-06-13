<script>
  export let peer;
  export let onSelect;
  export let onDropdownItemRender;

  const handleSelect = () => {
    onSelect(peer);
  };
</script>

<p class="dropdown-item" on:click={handleSelect}>
  {onDropdownItemRender(peer)}
</p>

<style lang="scss">
  @import "src/styles/variables";

  .dropdown-item {
    &:hover {
      cursor: pointer;
      background-color: $color-light-gray;
    }
  }
</style>
