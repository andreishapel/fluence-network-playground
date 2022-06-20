<script>
  export let title;
  export let options;

  let selected = options[0].value;
</script>

<aside class={`controls ${$$props.class}`}>
  <header class="header">
    {title}
  </header>
  {#each options as {name, id, label, value}}
    <div>
      <label class="radio" for={id}>{label}</label>
      <input name={name} id={id} value={value} bind:group={selected} type="radio">
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
