<script>
  import base64 from 'base-64';
  import Block from '@components/layout/block/block.svelte';
  import Field from '@components/form/field/field.svelte';
  import File from '@components/form/file/file.svelte';
  import Form from '@components/form/form/form.svelte';
  import Notification from '@components/notification/notification.svelte';
  import Tabs from '@components/tabs/tabs.svelte';
  import NetworkStore from '@store/network.store';
  import { uploadImage } from '@aqua/ipfs/ipfs';

  $: networkStatus = NetworkStore.store;
  let isResponseNotificationOpen = false;
  let errorMessage;

  const tabs = ['Upload', 'Download'];
  let selectedTabIndex;
  const onTabSelect = (tabIndex) => selectedTabIndex = tabIndex;

  const send = async (data) => {
    isResponseNotificationOpen = false;

    try {
      const { relayPeerId } = $networkStatus;
      const { serviceId, upload } = data;

      const base64Image = base64.encode(upload);
      await uploadImage(relayPeerId, serviceId, base64Image);
    } catch ({ message }) {
      errorMessage = message;
    }
  };
</script>

<Block class="ipfs" title="IPFS Integration" size="large">
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
    <Tabs tabs={tabs} onSelect={onTabSelect}></Tabs>
    {#if selectedTabIndex === 0}
      <File label="Choose a '.png' image to upload to IPFS"></File>
    {:else if selectedTabIndex === 1}
      WIP...
    {/if}
    <button class="button is-primary">Send</button>
  </Form>
</Block>

<style lang="scss">
  :global(.controls) {
    margin: 2px 0 !important;
  }

  :global(.file-label) {
    margin: 0 auto;
  }

  .button {
    width: 200px;
    margin: 20px auto;
    display: block;
  }
</style>
