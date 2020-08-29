/**
 * @fileoverview
 * Common mobx store.
 * Typically used to get some data from particular endpoint
 * If you want more complex logic with lots of states (fetch, put, path, delete etc.)
 *    don't you use that store. Write your own from scratch to avoid confusion
 */

import { action, computed, observable } from "mobx";
import { Observable, Subscription } from "rxjs";
import { message } from "antd";

interface ErrorLike {
  message: string
}

class SubscriptionStore {
  protected subscription: Subscription | undefined;

  @action.bound
  protected unsubscribe() {
    this.subscription?.unsubscribe();
    this.subscription = undefined;
    return this
  }
}

export class FetchStore extends SubscriptionStore {
  @observable state: "idle" | "pending" | "done" | "error" = "idle";
  @observable error: ErrorLike | undefined; // TODO @ikkuznetsov temp hack for support ApiError and Error interfaces. Needs refactor

  @computed get isLoading(): boolean {
    return this.state === "pending";
  }

  @action.bound
  protected beforeLoad() {
    this.state = "pending";
    this.error = undefined;
    return this
  }

  @action.bound
  protected errorHandler(error: ErrorLike) {
    this.state = "error";
    this.error = error;
  }

  @action.bound clear(): void {
    this.state = "idle";
    this.error = undefined;

    this.unsubscribe()
  }
}

export class MutationStore<T> extends SubscriptionStore {
  constructor(
    private successMessage: string | ((result: T) => string) = "Запрос выполнен успешно!",
    private errorMessage: string | ((error: ErrorLike) => string) = "При выполнении запроса произошла ошибка"
  ) {
    super();
  }

  @observable state: "idle" | "pending" = "idle";

  @computed get isMutating() {
    return this.state === "pending"
  }

  @action.bound
  protected mutateResource(observable$: Observable<T>): void {
    this.state = "pending";

    this.subscription = observable$.subscribe(
      this.successHandler,
      this.errorHandler
    )
  }

  @action.bound
  private successHandler(result: T) {
    this.state = "idle"

    message.success(
      typeof this.successMessage === "string" ? this.successMessage : this.successMessage(result)
    )
  }

  @action.bound
  private errorHandler(error: Error) {
    this.state = "idle"

    message.error(
      typeof this.errorMessage === "string" ? this.errorMessage : this.errorMessage(error)
    )
  }

  @action.bound clear(): void {
    this.state = "idle";

    this.unsubscribe()
  }
}
