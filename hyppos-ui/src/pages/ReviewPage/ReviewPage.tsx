import React from "react";
import ReactDOM from "react-dom";
import classes from "./classes.module.scss";
import { Editor, EditorConfiguration } from "codemirror"
import { Controlled as CodeMirror } from "react-codemirror2"

import "codemirror/lib/codemirror.css"
import "codemirror/theme/ayu-mirage.css";


import "codemirror/mode/jsx/jsx";
import "codemirror/mode/javascript/javascript";
import "codemirror/mode/go/go";
import "codemirror/mode/php/php";
import "codemirror/mode/python/python";
import "codemirror/mode/rust/rust";


import { fixtureComments } from "./ReviewPage.fixture";
import { Comments } from "./components/Comments";
import { observer } from "mobx-react-lite";
import { useReviewPageStore } from "./store/ReviewPageStore";
import { Card, Col, Row, Spin, Tree } from "antd";
import { TreeItem } from "../../api";
import { CommentForm } from "./components/Comment";
import { useRootStore } from "../../store/RootStore";


function makeOptions(file: string) {
  let mode

  switch (file.slice(-2)) {
    case "rs":
      mode = "rust"
      break
    case "js":
      mode = "javascript"
      break
    case "go":
      mode = "go"
      break
    case "hp":
      mode = "php"
      break
    case "py":
      mode = "python"
      break
    default:
      mode = "markdown"
  }


  const options: EditorConfiguration = {
    theme: "ayu-mirage",
    lineNumbers: true,
    mode: {
      name: mode
    },
    viewportMargin: Infinity
  }


  return options
}

function makeLineWidget(editor: Editor, line: number, widget: ((reset: () => void) => JSX.Element)) {
  const el = document.createElement("div")

  const instance = editor.addLineWidget(line, el, { coverGutter: true, handleMouseEvents: false })

  ReactDOM.render(widget(() =>{
    instance.clear()
    ReactDOM.unmountComponentAtNode(el)
  }), el)
}

function editorDidMountHandler(editor: Editor) {
  Object.entries(fixtureComments).forEach(([line, comments], index) => {
    const el = document.createElement("div")
    el.setAttribute("id", "file-comments-" + index)

    editor.addLineWidget(Number.parseInt(line) - 1, el, { coverGutter: true })

    ReactDOM.render(<Comments comments={comments}/>, el)
  })
}

interface MappedTreeItem {
  title: string
  key: string
  isLeaf?: boolean
  children?: MappedTreeItem[]
}

function toComponentTreeDataStructure(items: TreeItem[]) {
  function recursiveMapper(item: TreeItem): MappedTreeItem {
    const chunk = { title: item.path, key: item.sha, isLeaf: item.type === "File" }

    if (!item.children) {
      return chunk
    }

    return { ...chunk, children: item.children.map(recursiveMapper) }
  }

  return items.map(recursiveMapper)
}

interface ReviewPageProps {
  projectId: string
}

export const ReviewPage = observer(
  function ReviewPage() {
    const { authStore: { userName } } = useRootStore()
    const { rootContent, fileContent } = useReviewPageStore()

    React.useEffect(() => {
      rootContent.fetchRoot("tech-tasks", "master")

      return () => {
        rootContent.clear()
      }
    }, [rootContent])

    return (
      <div className={classes.root}>
        <Row>
          <Col span={9}>
            <Card style={{ height: "100%" }}>
              {rootContent.data && (
                // WARN! Здесь тонна ошибок, но нет ни времени, ни сил исправлять
                <Tree.DirectoryTree
                  multiple
                  autoExpandParent={false}
                  onSelect={(keys, node) => {
                    !node.node.isLeaf ?
                      rootContent.fetchChild("tech-tasks", node.node.key.toString())
                      : fileContent.fetchFileContent("tech-tasks", node.node.key.toString(), node.node.title?.toString() || "")
                  }}
                  treeData={toComponentTreeDataStructure(rootContent.data)}
                />
              )}
            </Card>
          </Col>

          <Col span={15}>
            {fileContent.isLoading ? (
              <Spin tip="Loading...">
                <div/>
              </Spin>
            ) : (
              <CodeMirror
                className={classes.codemirror}
                value={fileContent.data?.src || ""}
                options={makeOptions(fileContent.data?.name || "")}
                // editorDidMount={editorDidMountHandler}
                onGutterClick={(editor, lineNumber) =>
                  makeLineWidget(
                    editor,
                    lineNumber,
                    (reset) => <CommentForm userName={userName || "Anonymous"} reset={reset}/>
                  )
                }
                onBeforeChange={() => undefined}
              />
            )}

          </Col>
        </Row>
      </div>
    )
  }
)
