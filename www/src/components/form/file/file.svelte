<script>
  export let label;
  export let name;
</script>

<div class={`file has-name is-normal ${$$props.class}`}>
  <label class="file-label">
    <input class="file-input" name={name} accept="image/png" type="file">
    <span class="file-cta">
      <span class="file-icon">
        <i class="fas fa-upload"></i>
      </span>
      <span class="file-label">
        {label}
      </span>
    </span>
    <span class="file-name">
      Screen Shot 2017-07-29 at 15.54.25.png
    </span>
  </label>
</div>

<style lang="scss">
  .file {
    margin: 6px 0 4px 0;
  }
</style>
