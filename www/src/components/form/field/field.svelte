<script>
  export let label;
  export let name;
  export let id;
  export let isDisabled = false;
</script>

<div class={$$props.class}>
  <label class="label" for={id}>{label}</label>
  <input class="input" name={name} id={id} placeholder={label} disabled={isDisabled} type="text">
</div>

<style lang="scss">
  .label {
    margin: 6px 0 4px 0;
    font-weight: bold;
  }
</style>
