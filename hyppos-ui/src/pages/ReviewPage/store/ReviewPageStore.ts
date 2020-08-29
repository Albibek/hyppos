import { FetchStore } from "../../../store/Base/Base";
import { action, observable, runInAction } from "mobx";
import { api, TreeItem } from "../../../api";
import { makeStoreHook } from "../../../helpers/mobx";
import { forkJoin } from "rxjs";


class RootContentStore extends FetchStore {
  @observable data?: TreeItem[]

  @action.bound fetchRoot(repoName: string, branchName: string) {
    super.beforeLoad()
    this.data = undefined

    this.subscription = api.getRepoRoot(repoName, branchName)
      .subscribe(
        (result) => {
          runInAction(() => {
            this.data = result.data.items
            this.state = "done"
          })
        },
        this.errorHandler
      );

    return this.subscription
  }


  @action.bound fetchChild(repoName: string, path: string[]) {
    function findNode(tree: TreeItem[], hashes: string[]): TreeItem | undefined {
      const [sha, ...rest] = hashes

      const result = tree.find(it => it.sha === sha)


      if (rest.length === 0) {
        return result
      }

      if (!result || !result.children) {
        return
      }

      return findNode(result.children, rest)
    }

    const dirHash = path[path.length - 1]

    // @ts-ignore
    const target = findNode(this.data, path)


    if (dirHash && target) {
      this.subscription = api.getRepoDirContent(repoName, dirHash)
        .subscribe(
          (result) => {
            runInAction(() => {
              target.children = result.data.items
            })
          },
          this.errorHandler
        );

      return this.subscription
    }
  }


  @action.bound clear() {
    super.clear()
    this.data = undefined
  }
}

class FileContentStore extends FetchStore {
  @observable data?: { name: string, src: string, comments: Comment[] }

  @action.bound fetchFileContent(repoName: string, fileHash: string, fileName: string) {
    super.beforeLoad()
    this.data = undefined

    this.subscription = forkJoin({
      $src: api.getRepoFileContent(repoName, fileHash),
      $comments: api.getRepoFileComments(fileHash)
    })
      .subscribe(
        (result) => {
          runInAction(() => {
            // @ts-ignore
            this.data = { name: fileName, src: result.$src.data, comments: result.$comments.data }
            this.state = "done"
          })
        },
        this.errorHandler
      );

    return this.subscription
  }
}

export class ReviewPageStore extends FetchStore {
  rootContent: RootContentStore
  fileContent: FileContentStore

  constructor() {
    super();

    this.rootContent = new RootContentStore()
    this.fileContent = new FileContentStore()
  }
}

export const useReviewPageStore = makeStoreHook(new ReviewPageStore())
