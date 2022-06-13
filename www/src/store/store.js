export default class Store {
  constructor(store) {
    this.store = store;
    this.defaultStore = Object.assign({}, store);
  }

  reset() {
    const { store, defaultStore } = this;
    store.set(defaultStore);
  }
};
