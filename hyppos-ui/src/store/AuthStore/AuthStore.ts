import { action, observable, runInAction } from "mobx"
import { api } from "../../api"
import { persist } from "mobx-persist"
import { message } from "antd";

export class AuthStore {
  @observable loading = false
  @persist @observable isLoggedIn: boolean | undefined

  @action.bound login() {
    this.isLoggedIn = true
  }

  @action.bound logout() {
    this.loading = true

    api.logout().subscribe(
      result => runInAction(() => {
        this.loading = false

        if (result.status === 200) {
          this.isLoggedIn = false
        } else {
          message.error("Не удалось разлогиниться. Попробуйте еще раз!")
        }
      })
    )
  }
}
