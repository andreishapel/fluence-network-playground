export default class Store {
  constructor(store) {
    this.store = store;
    this.defaultStore = { ...store };
  }

  reset() {
    const { store } = this;
    const { defaultStore } = this;
    store.set(defaultStore);
  }
}
