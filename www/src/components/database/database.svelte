<script>
  import noop from 'lodash/noop';
  import { onMount } from 'svelte';
  import Block from '@components/layout/block/block.svelte';
  import Field from '@components/form/field/field.svelte';
  import Form from '@components/form/form/form.svelte';
  import Radio from '@components/form/radio/radio.svelte';
  import Notification from '@components/notification/notification.svelte';
  import NetworkStore from '@store/network.store';
  import { registerDatabase, init, insert, read } from '@aqua/database/database';

  const operationsOnDatabase = [{
    name: 'initAction',
    id: 'init',
    label: 'Initialize DB',
    value: 'init',
  }, {
    name: 'insertAction',
    id: 'insert',
    label: 'Insert (by FIRST NAME and LAST NAME)',
    value: 'insert',
  }, {
    name: 'readAction',
    id: 'read',
    label: 'Read (by FIRST NAME)',
    value: 'read',
  }, {
    name: 'updateAction',
    id: 'update',
    label: 'Update (by FIRST NAME)',
    value: 'update',
  }, {
    name: 'deleteAction',
    id: 'delete',
    label: 'Delete (by FIRST NAME)',
    value: 'delete',
  }];

  $: networkStatus = NetworkStore.store;
  $: isLastNameRadioDisabled = operationsOnDatabaseSelected === 'read' || operationsOnDatabaseSelected === 'delete';
  let isResponseNotificationOpen = false;
  let operationsOnDatabaseSelected;
  let errorMessage;

  const send = async (data) => {
    const { relayPeerId } = $networkStatus;
    const { serviceId, tableName, firstName, lastName, initAction, insertAction, readAction, updateAction, deleteAction } = data;

    const databaseAction = initAction || insertAction || readAction || updateAction || deleteAction;
    switch (databaseAction) {
      case initAction:
        const a1 = await init(relayPeerId, serviceId, tableName);
        console.log('1', a1);
        break;
      case insertAction:
        const a2 = await insert(relayPeerId, serviceId, tableName, firstName, lastName);
        console.log('2', a2);
        break;
      case readAction:
        const a3 = await read(relayPeerId, serviceId, tableName, firstName);
        console.log('3', a3);
        break;
    }
  };

  const operationsOnDatabaseSelect = value => operationsOnDatabaseSelected = value;

  onMount(() => {
    registerDatabase({
      init: noop,
      insert: noop,
      read: noop,
    });
  });
</script>

<Block class="database" title="CRUD operations on SQLite database" size="large">
  <Notification title="Description" isInfo={true} isOpen={true}>
    <p>Description</p>
  </Notification>
  <Notification title="Result" isSuccess={true} isOpen={isResponseNotificationOpen}>
    <p>Response.</p>
  </Notification>
  <Notification title="Error" isError={true} isOpen={!!errorMessage}>
    {errorMessage}
  </Notification>
  <Form onSubmit={send}>
    <Field label="Service ID" name="serviceId"></Field>
    <Field label="Table name (DB value)" name="tableName"></Field>
    <Field label="First name (DB value)" name="firstName"></Field>
    <Field label="Last name (DB value)" name="lastName" isDisabled={isLastNameRadioDisabled}></Field>
    <Radio title="Select operation" options={operationsOnDatabase} onSelect={operationsOnDatabaseSelect}></Radio>
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
