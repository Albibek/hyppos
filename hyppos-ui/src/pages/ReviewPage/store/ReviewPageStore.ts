import { FetchStore, MutationStore } from "../../../store/Base/Base";
import { action, observable, runInAction } from "mobx";
import { api, NewComment, TreeItem } from "../../../api";
import { makeStoreHook } from "../../../helpers/mobx";
import { forkJoin } from "rxjs";

function findNode(hash: string, tree: TreeItem[]): TreeItem | undefined {
  let i = 0, found;

  for (; i < tree.length; i++) {
    const current = tree[i]

    if (current.sha === hash) {
      return current;
    } else if (current.children !== undefined) {
      found = findNode(hash, current.children);

      if (found) {
        return found;
      }
    }
  }

  return
}

class InsertCommentStore extends MutationStore<any> {
  @action.bound insertComment(newComment: NewComment) {
    this.mutateResource(
      api.insertComment(newComment)
    )
  }
}

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


  @action.bound fetchChild(repoName: string, path: string) {
    // @ts-ignore
    const target = findNode(path, this.data)

    if (path && target) {
      this.subscription = api.getRepoDirContent(repoName, path)
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
            this.data = { name: fileName, src: result.$src.data, comments: [] } // result.$comments.data
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
  insertCommentStore: InsertCommentStore

  constructor() {
    super();

    this.rootContent = new RootContentStore()
    this.fileContent = new FileContentStore()
    this.insertCommentStore = new InsertCommentStore()
  }
}

export const useReviewPageStore = makeStoreHook(new ReviewPageStore())
