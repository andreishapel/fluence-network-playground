<script>
  import noop from 'lodash/noop';
  import { onMount } from 'svelte';
  import Block from '@components/layout/block/block.svelte';
  import Field from '@components/form/field/field.svelte';
  import Form from '@components/form/form/form.svelte';
  import Radio from '@components/form/radio/radio.svelte';
  import Notification from '@components/notification/notification.svelte';
  import NetworkStore from '@store/network.store';
  import { CREATE_OPERATION, INSERT_OPERATION, READ_OPERATION, UPDATE_OPERATION, DELETE_OPERATION, OPERATIONS_ON_DATABASE } from '@constants/database';
  import { registerDatabase, create, insert, read } from '@aqua/database/database';

  $: networkStatus = NetworkStore.store;
  $: isCreateOperationSelected = operationsOnDatabaseSelected === CREATE_OPERATION;
  $: isInsertOperationSelected = operationsOnDatabaseSelected === INSERT_OPERATION;
  $: isReadOperationSelected = operationsOnDatabaseSelected === READ_OPERATION;
  $: isUpdateOperationSelected = operationsOnDatabaseSelected === UPDATE_OPERATION;
  $: isDeleteOperationSelected = operationsOnDatabaseSelected === DELETE_OPERATION;
  $: isFirstNameRadioDisabled = isCreateOperationSelected && (!isInsertOperationSelected || !isReadOperationSelected || !isDeleteOperationSelected);
  $: isLastNameRadioDisabled = isCreateOperationSelected && (isInsertOperationSelected || isReadOperationSelected || isDeleteOperationSelected);
  let isResponseNotificationOpen = false;
  let operationsOnDatabaseSelected;
  let errorMessage;

  const send = async (data) => {
    const { relayPeerId } = $networkStatus;
    const { serviceId, tableName, firstName, lastName, createAction, insertAction, readAction, updateAction, deleteAction } = data;

    const databaseAction = createAction || insertAction || readAction || updateAction || deleteAction;
    switch (databaseAction) {
      case createAction:
        const a1 = await create(relayPeerId, serviceId, tableName);
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
      create: noop,
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
    <Field label="First name (DB value)" name="firstName" isDisabled={isFirstNameRadioDisabled}></Field>
    <Field label="Last name (DB value)" name="lastName" isDisabled={isLastNameRadioDisabled}></Field>
    <Radio title="Select operation" options={OPERATIONS_ON_DATABASE} onSelect={operationsOnDatabaseSelect}></Radio>
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
