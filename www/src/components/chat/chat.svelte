<script>
  import Block from '@components/layout/block/block.svelte';
  import Form from '@components/form/form/form.svelte';
  import Field from '@components/form/field/field.svelte';
  // import { registerChat, say } from '../../_aqua/chat/chat';

  const sendMessage = (data) => {
    const { targetPeerId, targetRelayPeerId, messageFromYou } = data;
  };
</script>

<Block title="P2P Chat" size="medium">
  <Form onSubmit={sendMessage}>
    <Field label="Target peer id" name="targetPeerId" />
    <Field label="Target relay peer id" name="targetRelayPeerId" />
    <Field label="Message from you" name="messageFromYou" />
    <Field label="Message to you" name="messageToYou" isDisabled={true} />
    <button class="button is-primary">Send</button>
  </Form>
</Block>

<style lang="scss">
  .button {
    width: 200px;
    margin: 10px auto;
    display: block;
  }
</style>
