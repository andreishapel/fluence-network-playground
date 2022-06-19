<script>
  export let options;

  let selected = options[0].value;
</script>

<div class={`control ${$$props.class}`}>
  {#each options as {name, id, label, value}}
    <div>
      <label class="radio" for={id}>{label}</label>
      <input name={name} id={id} value={value} bind:group={selected} type="radio">
    </div>
  {/each}
</div>
