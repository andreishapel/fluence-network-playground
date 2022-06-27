export const CREATE_OPERATION = 'create';
export const INSERT_OPERATION = 'insert';
export const READ_OPERATION = 'read';
export const UPDATE_OPERATION = 'update';
export const DELETE_OPERATION = 'delete';

export const OPERATIONS_ON_DATABASE = [{
  name: 'createAction',
  label: 'Create',
  id: CREATE_OPERATION,
  value: CREATE_OPERATION,
}, {
  name: 'insertAction',
  label: 'Insert (by FIRST NAME and LAST NAME)',
  id: INSERT_OPERATION,
  value: INSERT_OPERATION,
}, {
  name: 'readAction',
  label: 'Read (by FIRST NAME)',
  id: READ_OPERATION,
  value: READ_OPERATION,
}, {
  name: 'updateAction',
  label: 'Update (by FIRST NAME)',
  id: UPDATE_OPERATION,
  value: UPDATE_OPERATION,
}, {
  name: 'deleteAction',
  label: 'Delete (by FIRST NAME)',
  id: DELETE_OPERATION,
  value: DELETE_OPERATION,
}];
