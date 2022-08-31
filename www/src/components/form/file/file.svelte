<script>
  export let label;

  let selectedFileName = 'Not selected';

  const handleSelect = (event) => {
    const {
      target: {
        files,
      },
    } = event

    const firstFile = files[0];
    selectedFileName = firstFile.name;
  };
</script>

<div class="file has-name">
  <label class="file-label">
    <input class="file-input" type="file" accept="image/png" multiple="false" name="upload" on:change={handleSelect}>
    <span class="file-cta">
      <span class="file-icon">
        <i class="fas fa-upload"></i>
      </span>
      <span class="file-label">
        { label }
      </span>
    </span>
    <span class="file-name">
      { selectedFileName }
    </span>
  </label>
</div>

<style lang="scss">
  .file {
    margin: 6px 0 4px 0;
  }
</style>
