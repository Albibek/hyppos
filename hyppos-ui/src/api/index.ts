/* eslint-disable @typescript-eslint/camelcase */
import { gatewayClient } from "./client";
import { from } from "rxjs";
import { map } from "rxjs/operators";

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
  return from(gatewayClient.get<string[]>("/comments", { params: { file_id: fileHash } }))
}

export interface NewComment {
  lineNo: number
  commitId: string
  fileId: string
  projectId: string
  message: string
}

function insertComment(newComment: NewComment) {
  return from(gatewayClient.post<string[]>("/comments", {
    line_no: newComment.lineNo,
    commit_id: newComment.commitId,
    file_id: newComment.fileId,
    project_id: newComment.projectId,
    message: newComment.message,
  }))
}

export interface Project {
  id: string
  name: string
  externalId: string
  userId: string
  createdAt: Date
}


function getProjects() {
  return from(gatewayClient.get<any>("/projects")).pipe(
    map(res => res.data.map((it: any) => ({
      id: it.id,
      name: it.name,
      externalId: it.external_id,
      userId: it.user_id,
      createdAt: new Date(it.created_at)
    })) as Project[])
  )
}

function insertProject(newProject: { externalId: number }) {
  return from(gatewayClient.post("/projects", {
    external_id: newProject.externalId,
  }))
}

export const api = {
  logout,
  getMyRepos,
  getRepoRoot,
  getProjects,
  getRepoDirContent,
  getRepoFileContent,
  getRepoFileComments,
  insertComment,
  insertProject
}
