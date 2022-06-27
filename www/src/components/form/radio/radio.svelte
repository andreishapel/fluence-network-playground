<script>
  import get from 'lodash/get';

  export let title;
  export let options;
  export let selected = options[0].value;
  export let onSelect;

  const select = (data) => {
    if (onSelect) {
      const value = get(data, ['target', 'value']);
      onSelect(value);
    }
  };
</script>

<aside class={`controls ${$$props.class}`}>
  <header class="header">
    {title}
  </header>
  {#each options as {name, id, label, value}}
    <div>
      <label class="radio" for={id}>{label}</label>
      <input name={name} id={id} value={value} bind:group={selected} on:change|preventDefault={select} type="radio">
    </div>
  {/each}
</aside>

<style lang="scss">
  .controls {
    margin: 4px 0 10px 0;
  }

  .header {
    font-weight: bold;
  }
</style>
