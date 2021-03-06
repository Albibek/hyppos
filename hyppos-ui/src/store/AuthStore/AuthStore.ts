import { action, observable, runInAction } from "mobx"
import { api } from "../../api"
import { persist } from "mobx-persist"
import { message } from "antd";

export class AuthStore {
  @observable loading = false
  @persist @observable userId: string | undefined
  @persist @observable userName: string | undefined

  @action.bound login(userId: string, userName: string) {
    this.userId = userId
    this.userName = userName
  }

  @action.bound logout() {
    this.loading = true

    api.logout().subscribe(
      result => runInAction(() => {
        this.loading = false

        if (result.status === 200) {
          this.userId = undefined
          this.userName = undefined
        } else {
          message.error("Не удалось разлогиниться. Попробуйте еще раз!")
        }
      })
    )
  }
}
