<script>
  export let label;
  export let name;
  export let id;
  export let value;
  export let readonly = false;
</script>

<div class={$$props.class}>
  <label class="label" for={id}>{label}</label>
  <textarea class="textarea" name={name} id={id} placeholder={label} type="text" readonly={readonly} bind:value></textarea>
</div>

<style lang="scss">
  .label {
    margin: 6px 0 4px 0;
    font-weight: bold;
  }
</style>
