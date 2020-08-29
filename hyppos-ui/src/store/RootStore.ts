import { configure, observable, runInAction } from "mobx"
import { create, IHydrateResult } from "mobx-persist";
import { AuthStore } from "./AuthStore/AuthStore";
import { makeStoreHook } from "../helpers/mobx";

configure({
  enforceActions: "always" // all changes to observables must be initiated by action methods only
})

const hydrate = create({ jsonify: true })

class RootStore {
  readonly authStore: AuthStore

  @observable isAppLoading = true

  constructor() {
    this.authStore = new AuthStore()

    const persistentStores: IHydrateResult<unknown>[] = [
      hydrate("auth", this.authStore),
    ]

    // sync load of mandatory dependencies
    Promise
      .all(persistentStores)
      .then(() => {
        runInAction("finishAppLoading", () => {
          this.isAppLoading = false
        })
      })
  }
}

export const useRootStore = makeStoreHook(new RootStore())
