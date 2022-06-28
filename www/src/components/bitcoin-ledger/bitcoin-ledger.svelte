<script>
  import { onMount } from 'svelte';
  import Block from '@components/layout/block/block.svelte';
  import Field from '@components/form/field/field.svelte';
  import Form from '@components/form/form/form.svelte';
  import Notification from '@components/notification/notification.svelte';
  import NetworkStore from '@store/network.store';

  $: networkStatus = NetworkStore.store;
  let isResponseNotificationOpen = false;
  let errorMessage;

  const send = async (data) => {
    isResponseNotificationOpen = false;

    try {
      const { relayPeerId } = $networkStatus;
      const { serviceId, transactionId } = data;
    } catch ({ message }) {
      errorMessage = message;
    }
  };

  onMount(() => {});
</script>

<Block class="bitcoin-ledger" title="Bitcoin ledger" size="large">
  <Notification title="Description" isInfo={true} isOpen={true}>
    <p>Description</p>
  </Notification>
  <Notification title="Result" isSuccess={true} isOpen={isResponseNotificationOpen}>
    <p>Result</p>
  </Notification>
  <Notification title="Error" isError={true} isOpen={!!errorMessage}>
    {errorMessage}
  </Notification>
  <Form onSubmit={send}>
    <Field label="Service ID" name="serviceId"></Field>
    <Field label="Transaction ID" name="transactionId"></Field>
    <button class="button is-primary">Send</button>
  </Form>
</Block>

<style lang="scss">
  :global(.controls) {
    margin: 2px 0 !important;
  }

  .button {
    width: 200px;
    margin: 10px auto;
    display: block;
  }
</style>
