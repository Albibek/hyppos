import { useLocalStore } from "mobx-react-lite";

export function makeStoreHook<T>(store: T) {
  return () => useLocalStore(() => store)
}
