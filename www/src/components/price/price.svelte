<script>
  import noop from 'lodash/noop';
  import { onMount } from 'svelte';
  import Block from '@components/layout/block/block.svelte';
  import Field from '@components/form/field/field.svelte';
  import Form from '@components/form/form/form.svelte';
  import Radio from '@components/form/radio/radio.svelte';
  import Notification from '@components/notification/notification.svelte';
  import NetworkStore from '@store/network.store';
  import { registerPrice, fetchPrices } from '@aqua/price/price';

  const cryptoTokens = [{
    name: 'btc',
    id: 'btc',
    label: 'BTC',
    value: 'btc',
  }, {
    name: 'eth',
    id: 'eth',
    label: 'ETH',
    value: 'eth',
  }];
  const fiatCurrencies = [{
    name: 'usd',
    id: 'usd',
    label: 'USD',
    value: 'usd',
  }, {
    name: 'eur',
    id: 'eur',
    label: 'EUR',
    value: 'eur',
  }];

  $: networkStatus = NetworkStore.store;
  let errorMessage;

  const send = async (data) => {
    try {
      const { relayPeerId } = $networkStatus;
      const { serviceId } = data;
      const response = await fetchPrices(relayPeerId, serviceId, 'http://google.com');
      console.log('@', data);
      console.log('#', response);
    } catch ({ message }) {
      errorMessage = message;
    }
  };

  onMount(() => {
    registerPrice({
      download: noop,
    });
  });
</script>

<Block class="price" title="Price Oracle" size="large">
  <Notification title="Description" isInfo={true} isOpen={true}>
    <p>Description</p>
  </Notification>
  <Notification title="Error" isError={true} isOpen={!!errorMessage}>
    {errorMessage}
  </Notification>
  <Form onSubmit={send}>
    <Field label="Service ID" name="serviceId"></Field>
    <Radio title="Select crypto currency" options={cryptoTokens}></Radio>
    <Radio title="Select fiat currency" options={fiatCurrencies}></Radio>
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
