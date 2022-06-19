<script>
  import { onMount } from 'svelte';
  import { Observable } from 'rxjs';
  import Block from '@components/layout/block/block.svelte';
  import Field from '@components/form/field/field.svelte';
  import Form from '@components/form/form/form.svelte';
  import TextArea from '@components/form/text-area/text-area.svelte';
  import Notification from '@components/notification/notification.svelte';
  import { registerChat, sendMessage } from '@aqua/chat/chat';

  let messages = new Observable();
  let errorMessage;

  const send = async (data) => {
    try {
      const { targetPeerId, targetRelayPeerId, messageFromYou } = data;
      await sendMessage(targetPeerId, targetRelayPeerId, messageFromYou);
    } catch ({ message }) {
      errorMessage = message;
    }
  };

  onMount(() => {
    messages = new Observable((observer) => {
      registerChat({
        message: (from, text) => {
          const timeStamp = new Date().toUTCString();
          const toPrint = `Message: ${text}\nFrom: ${from}\nWhen: ${timeStamp}\n`;
          observer.next(toPrint);
        },
      });
    });
  });
</script>

<Block class="chat" title="P2P Chat" size="large">
  <Notification class="notification-error" title="Error" isError={true} isOpen={!!errorMessage}>
    {errorMessage}
  </Notification>
  <Form onSubmit={send}>
    <Field label="Target peer id" name="targetPeerId" id="targetPeerId" />
    <Field label="Target relay peer id" name="targetRelayPeerId" id="targetRelayPeerId" />
    <Field label="Message from you" name="messageFromYou" id="messageFromYou" />
    <TextArea label="Message to you" name="messageToYou" id="messageToYou" value={$messages} readonly={true}></TextArea>
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
