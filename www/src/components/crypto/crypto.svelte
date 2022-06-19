<script>
  import noop from 'lodash/noop';
  import { onMount } from 'svelte';
  import Block from '@components/layout/block/block.svelte';
  import Field from '@components/form/field/field.svelte';
  import Form from '@components/form/form/form.svelte';
  import Radio from '@components/form/radio/radio.svelte';
  import Notification from '@components/notification/notification.svelte';
  import NetworkStore from '@store/network.store';
  import { registerCrypto, hashText } from '@aqua/crypto/crypto';

  const algorithms = [{
    name: 'md5',
    id: 'md5',
    label: 'MD5',
    value: 'md5',
  }, {
    name: 'sha256',
    id: 'sha256',
    label: 'SHA256',
    value: 'sha256',
  }];

  $: networkStatus = NetworkStore.store;
  let isResponseNotificationOpen = false;
  let algorithmResponse;
  let hashedTextResponse;
  let originalTextResponse;
  let errorMessage;

  const send = async (data) => {
    isResponseNotificationOpen = false;
    errorMessage = '';

    try {
      const { relayPeerId } = $networkStatus;
      const { serviceId, text, md5, sha256 } = data;

      const hashingAlgorithm = md5 || sha256;
      const response = await hashText(relayPeerId, serviceId, hashingAlgorithm, text);

      const {
        algorithm,
        hashed_text: hashedText,
        original_text: originalText,
      } = response;

      isResponseNotificationOpen = true;
      algorithmResponse = algorithm;
      hashedTextResponse = hashedText;
      originalTextResponse = originalText;
    } catch ({ message }) {
      errorMessage = message;
    }
  };

  onMount(() => {
    registerCrypto({
      hash_text: noop,
    });
  });
</script>

<Block class="chat" title="Cryptography" size="large">
  <Notification class="notification-result" title="Result" isSuccess={true} isOpen={isResponseNotificationOpen}>
    <p>Algorithm: {algorithmResponse}</p>
    <p>Hash(text): {hashedTextResponse}</p>
    <p>Text: {originalTextResponse}</p>
  </Notification>
  <Notification class="notification-error" title="Error" isError={true} isOpen={!!errorMessage}>
    {errorMessage}
  </Notification>
  <Form onSubmit={send}>
    <Field label="Service ID" name="serviceId" />
    <Field label="Text to hash" name="text" />
    <aside class="controls">
      <header class="header">
        Select cryptographic algorithm
      </header>
      <Radio options={algorithms}></Radio>
    </aside>
    <button class="button is-primary">Send</button>
  </Form>
</Block>

<style lang="scss">
  .controls {
    margin: 4px 0 10px 0;
  }

  .header {
    font-weight: bold;
  }

  .button {
    width: 200px;
    margin: 10px auto;
    display: block;
  }
</style>
