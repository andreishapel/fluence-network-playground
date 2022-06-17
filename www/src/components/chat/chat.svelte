<script>
  import { onMount } from 'svelte';
  import { Observable } from 'rxjs';
  import Block from '@components/layout/block/block.svelte';
  import Form from '@components/form/form/form.svelte';
  import Field from '@components/form/field/field.svelte';
  import TextArea from '@components/form/text-area/text-area.svelte';
  import { registerChat, sendMessage } from '../../_aqua/chat/chat';

  let messages = new Observable();

  const send = async (data) => {
    const { targetPeerId, targetRelayPeerId, messageFromYou } = data;
    await sendMessage(targetPeerId, targetRelayPeerId, messageFromYou);
  };

  onMount(() => {
    registerChat({
      message: (from, text) => {
        messages = new Observable((observer) => {
          const timeStamp = new Date().toUTCString();
          const toPrint = `Message: ${text}\nFrom: ${from}\nWhen: ${timeStamp}\n`;
          observer.next(toPrint);
        });
      },
    });
  });
</script>

<Block class="chat" title="P2P Chat" size="medium">
  <Form onSubmit={send}>
    <Field label="Target peer id" name="targetPeerId" />
    <Field label="Target relay peer id" name="targetRelayPeerId" />
    <Field label="Message from you" name="messageFromYou" />
    <TextArea label="Message to you" name="messageToYou1" value={$messages} readonly={true}></TextArea>
    <button class="button is-primary">Send</button>
  </Form>
</Block>

<style lang="scss">
  :global(.textarea)  {
    font-size: 12px;
  }

  .button {
    width: 200px;
    margin: 10px auto;
    display: block;
  }
</style>
