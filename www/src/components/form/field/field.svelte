<script>
  export let label;
  export let name;
</script>

<div class={$$props.class}>
  <label class="label" for={name}>{label}</label>
  <input class="input" placeholder={label} type="text">
</div>

<style lang="scss">
  .label {
    margin: 6px 0 4px 0;
    font-weight: bold;
  }
</style>
