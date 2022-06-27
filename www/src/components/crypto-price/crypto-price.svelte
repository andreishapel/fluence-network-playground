<script>
  import noop from 'lodash/noop';
  import capitalize from 'lodash/capitalize';
  import { onMount } from 'svelte';
  import Block from '@components/layout/block/block.svelte';
  import Field from '@components/form/field/field.svelte';
  import Form from '@components/form/form/form.svelte';
  import Radio from '@components/form/radio/radio.svelte';
  import Notification from '@components/notification/notification.svelte';
  import NetworkStore from '@store/network.store';
  import { CRYPTO_CURRENCIES, FIAT_CURRENCIES } from '@constants/crypto-price';
  import { registerCryptoPrice, getPrice } from '@aqua/crypto-price/crypto-price';

  $: networkStatus = NetworkStore.store;
  let isResponseNotificationOpen = false;
  let cryptoCurrencyResponse;
  let fiatCurrencyResponse;
  let priceResponse;
  let errorMessage;

  const send = async (data) => {
    isResponseNotificationOpen = false;

    try {
      const { relayPeerId } = $networkStatus;
      const { serviceId, btc, eth, usd, eur } = data;

      const cryptoCurrencyName = btc || eth;
      const fiatCurrencyName = usd || eur;

      const {
        crypto_currency: cryptoCurrency,
        fiat_currency: fiatCurrency,
        price,
      } = await getPrice(relayPeerId, serviceId, cryptoCurrencyName, fiatCurrencyName);

      isResponseNotificationOpen = true;
      cryptoCurrencyResponse = capitalize(cryptoCurrency);
      fiatCurrencyResponse = fiatCurrency.toUpperCase();
      priceResponse = price;
    } catch ({ message }) {
      errorMessage = message;
    }
  };

  onMount(() => {
    registerCryptoPrice({
      get_price: noop,
    });
  });
</script>

<Block class="crypto-price" title="Cryptocurrencies Price" size="large">
  <Notification title="Description" isInfo={true} isOpen={true}>
    <p>Demonstrates how to use an oracle service to make an HTTP request to <a href="https://www.coingecko.com/" target="_blank">CoinGecko</a> to get cryptocurrency price in specified fiat currency.</p>
    <p>To use it, enter the "serviceId" you received after deployment, and choose crypto currency and fiat currency.</p>
  </Notification>
  <Notification title="Result" isSuccess={true} isOpen={isResponseNotificationOpen}>
    <p>{cryptoCurrencyResponse} token price in {fiatCurrencyResponse} is {priceResponse}.</p>
  </Notification>
  <Notification title="Error" isError={true} isOpen={!!errorMessage}>
    {errorMessage}
  </Notification>
  <Form onSubmit={send}>
    <Field label="Service ID" name="serviceId"></Field>
    <Radio title="Select crypto currency" options={CRYPTO_CURRENCIES}></Radio>
    <Radio title="Select fiat currency" options={FIAT_CURRENCIES}></Radio>
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
