<script>
  import Block from '@components/layout/block/block.svelte';
  import Field from '@components/form/field/field.svelte';
</script>

<Block title="P2P Chat" size="medium">
  <Field label="Target peer id" name="targetPeerId" />
  <Field label="Target relay peer id" name="targetRelayPeerId" />
  <Field label="Message" name="message" />
  <button class="button is-primary">Send</button>
</Block>

<style lang="scss">
  .button {
    width: 200px;
    margin: 10px auto;
    display: block;
  }
</style>
