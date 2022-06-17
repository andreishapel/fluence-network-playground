<script>
  export let label;
  export let name;
  export let isDisabled = false;
</script>

<div class={$$props.class}>
  <label class="label" for={name}>{label}</label>
  <input class="input" name={name} placeholder={label} type="text" disabled={isDisabled}>
</div>

<style lang="scss">
  .label {
    margin: 6px 0 4px 0;
    font-weight: bold;
  }
</style>
