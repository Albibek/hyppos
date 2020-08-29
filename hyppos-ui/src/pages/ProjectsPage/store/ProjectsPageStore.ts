import { FetchStore } from "../../../store/Base/Base";
import { action, observable, runInAction } from "mobx";
import { api, Project, TreeItem } from "../../../api";
import { makeStoreHook } from "../../../helpers/mobx";
import { forkJoin } from "rxjs";


export class ProjectsPageStore extends FetchStore {
  @observable data?: Project[]

  @action.bound fetchProjects() {
    super.beforeLoad()
    this.data = undefined

    this.subscription = api.getProjects()
      .subscribe(
        (result) => {
          runInAction(() => {
            this.data = result
            this.state = "done"
          })
        },
        this.errorHandler
      );

    return this.subscription
  }

  @action.bound clear() {
    super.clear()
    this.data = undefined
  }
}

export const useProjectsPageStore = makeStoreHook(new ProjectsPageStore())
