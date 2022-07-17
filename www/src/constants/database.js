export const CREATE_OPERATION = 'create';
export const INSERT_OPERATION = 'insert';
export const READ_OPERATION = 'read';
export const DELETE_OPERATION = 'delete';

export const OPERATIONS_ON_DATABASE = [{
  name: 'createDBAction',
  label: 'Create',
  id: CREATE_OPERATION,
  value: CREATE_OPERATION,
}, {
  name: 'readDBAction',
  label: 'Read (ALL)',
  id: READ_OPERATION,
  value: READ_OPERATION,
}, {
  name: 'insertDBAction',
  label: 'Insert (by FIRST NAME and LAST NAME)',
  id: INSERT_OPERATION,
  value: INSERT_OPERATION,
}, {
  name: 'deleteDBAction',
  label: 'Delete (by ID)',
  id: DELETE_OPERATION,
  value: DELETE_OPERATION,
}];
