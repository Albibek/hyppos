import { FetchStore } from "../../../store/Base/Base";
import { action, observable, runInAction } from "mobx";
import { api } from "../../../api";
import { makeStoreHook } from "../../../helpers/mobx";


class RootContentStore extends FetchStore {
  @observable data?: any

  @action.bound fetchRoot(repoName: string, branchName: string) {
    super.beforeLoad()
    this.data = undefined

    this.subscription = api.getRepoRoot(repoName, branchName)
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


export class ReviewPageStore extends FetchStore {
  rootContent: RootContentStore

  constructor() {
    super();

    this.rootContent = new RootContentStore()
  }
}

export const useReviewPageStore = makeStoreHook(new ReviewPageStore())
