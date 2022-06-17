export default class Store {
  constructor(store) {
    this.store = store;
    this.defaultStore = Object.assign({}, store);
  }

  reset() {
    const store = this.store;
    const defaultStore = this.defaultStore;
    store.set(defaultStore);
  }
}
