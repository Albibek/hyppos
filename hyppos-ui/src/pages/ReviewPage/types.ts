type LineNumber = number

export interface Comment {
  id: string | number
  lineNo: number
  createdAt: Date
  message: string
  user: { name: string }
}


export type FileComments = Record<LineNumber, Comment[]>
