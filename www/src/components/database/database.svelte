<script>
  import noop from 'lodash/noop';
  import { onMount } from 'svelte';
  import Block from '@components/layout/block/block.svelte';
  import Field from '@components/form/field/field.svelte';
  import Form from '@components/form/form/form.svelte';
  import Radio from '@components/form/radio/radio.svelte';
  import Table from '@components/layout/table/table.svelte';
  import Notification from '@components/notification/notification.svelte';
  import NetworkStore from '@store/network.store';
  import { CREATE_OPERATION, INSERT_OPERATION, READ_OPERATION, DELETE_OPERATION, OPERATIONS_ON_DATABASE } from '@constants/database';
  import { registerDatabase, createAction, readAction, insertAction, deleteAction } from '@aqua/database/database';

  $: networkStatus = NetworkStore.store;
  $: isCreateOperationSelected = operationsOnDatabaseSelected === CREATE_OPERATION;
  $: isInsertOperationSelected = operationsOnDatabaseSelected === INSERT_OPERATION;
  $: isReadOperationSelected = operationsOnDatabaseSelected === READ_OPERATION;
  $: isDeleteOperationSelected = operationsOnDatabaseSelected === DELETE_OPERATION;
  $: isDatabaseIdFieldDidsabled = isCreateOperationSelected;
  $: isIdFieldDisabled = !isDeleteOperationSelected;
  $: isFirstNameRadioDisabled = !isInsertOperationSelected;
  $: isLastNameRadioDisabled = !isInsertOperationSelected;
  let tableColumns = ['ID', 'First name', 'Last Name'];
  let isResponseNotificationOpen = false;
  let operationsOnDatabaseSelected = CREATE_OPERATION;
  let databaseIdResponse;
  let databaseEntitiesResponse;
  let errorMessage;

  const send = async (data) => {
    isResponseNotificationOpen = false;
    databaseIdResponse = null;
    databaseEntitiesResponse = null;

    const { relayPeerId } = $networkStatus;
    const { serviceId, databaseId, id, firstName, lastName, createDBAction, insertDBAction, readDBAction, deleteDBAction } = data;

    const databaseIdAsInteger = Number(databaseId);
    const idAsInteger = Number(id);

    const databaseAction = createDBAction || insertDBAction || readDBAction || deleteDBAction;
    switch (databaseAction) {
      case createDBAction:
        databaseIdResponse = (await createAction(relayPeerId, serviceId)).database_id;
        break;
      case readDBAction:
        const a = await readAction(relayPeerId, serviceId, databaseIdAsInteger);
        console.log('#', a);
        break;
      case insertDBAction:
        databaseIdResponse = (await insertAction(relayPeerId, serviceId, databaseIdAsInteger, firstName, lastName)).database_id;
        break;
      case deleteDBAction:
        databaseIdResponse = (await deleteAction(relayPeerId, serviceId, databaseIdAsInteger, idAsInteger)).database_id;
        break;
    }

    isResponseNotificationOpen = true;
  };

  const operationsOnDatabaseSelect = (value) => {
    isResponseNotificationOpen = false;
    operationsOnDatabaseSelected = value;
  };

  onMount(() => {
    registerDatabase({
      create: noop,
      read: noop,
      insert: noop,
      delete: noop,
    });
  });
</script>

<Block class="database" title="CRUD operations on SQLite database" size="large">
  <Notification title="Description" isInfo={true} isOpen={true}>
    <p>Description</p>
  </Notification>
  <Notification title="Result" isSuccess={true} isOpen={isResponseNotificationOpen}>
    {#if isCreateOperationSelected }
      <p>Table successfully created.</p>
      <p>Database ID: {databaseIdResponse}</p>
    {/if}
    {#if isReadOperationSelected}
      <Table columns={tableColumns} rows={databaseEntitiesResponse}></Table>
    {/if}
    {#if isInsertOperationSelected}
      <p>Entity successfully added.</p>
    {/if}
    {#if isDeleteOperationSelected}
      <p>Entity successfully deleted.</p>
    {/if}
  </Notification>
  <Notification title="Error" isError={true} isOpen={!!errorMessage}>
    {errorMessage}
  </Notification>
  <Form onSubmit={send}>
    <Field label="Service ID" name="serviceId"></Field>
    <Field label="Database ID" name="databaseId" isDisabled={isDatabaseIdFieldDidsabled}></Field>
    <Field label="ID (DB value)" name="id" isDisabled={isIdFieldDisabled}></Field>
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

  :global(.table) {
    width: 100%;
    background: transparent;
  }

  .button {
    width: 200px;
    margin: 10px auto;
    display: block;
  }
</style>
