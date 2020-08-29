import { gatewayClient } from "./client";
import { from } from "rxjs";

function logout() {
  return from(gatewayClient.get("/auth/logout"))
}

function getMyRepos() {
  return from(gatewayClient.get("/gh/repos"))
}

export interface TreeItem {
  type: "Dir" | "File"
  path: string
  sha: string
  url: string
  size: number
  children?: TreeItem[]
}

function getRepoRoot(repoName: string, branchName: string) {
  return from(gatewayClient.get<{ items: TreeItem[] }>(`/gh/repos/${repoName}/branch/${branchName}`))
}

function getRepoDirContent(repoName: string, dirHash: string) {
  return from(gatewayClient.get(`/gh/repos/${repoName}/dirs/${dirHash}`))
}

function getRepoFileContent(repoName: string, fileHash: string) {
  return from(gatewayClient.get<string>(`/gh/repos/${repoName}/files/${fileHash}`))
}

function getRepoFileComments(fileHash: string) {
  // eslint-disable-next-line @typescript-eslint/camelcase
  return from(gatewayClient.get<string[]>("/comments", { params: { file_id: fileHash } }))
}

export const api = {
  logout, getMyRepos, getRepoRoot, getRepoDirContent, getRepoFileContent, getRepoFileComments
}
