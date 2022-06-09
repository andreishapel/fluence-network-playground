import { writable } from 'svelte/store';

export default writable({
  connectedTo: '',
  client: null,
  isConnected: false,
});
