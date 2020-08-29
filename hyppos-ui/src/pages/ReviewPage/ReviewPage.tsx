import React from "react";
import ReactDOM from "react-dom";
import classes from "./classes.module.scss";
import { Editor, EditorConfiguration } from "codemirror"
import { Controlled as CodeMirror } from "react-codemirror2"

import "codemirror/lib/codemirror.css"
import "codemirror/theme/ayu-mirage.css";

import "codemirror/mode/rust/rust";
import { fixtureComments, fixtureFile } from "./ReviewPage.fixture";
import { Comments } from "./components/Comments";
import { observer } from "mobx-react-lite";
import { useReviewPageStore } from "./store/ReviewPageStore";
import { Card, Col, Row } from "antd";


const defaultOptions: EditorConfiguration = {
  theme: "ayu-mirage",
  lineNumbers: true,
  mode: {
    name: "rust"
  },
  viewportMargin: Infinity
}

function editorDidMountHandler(editor: Editor) {
  Object.entries(fixtureComments).forEach(([line, comments], index) => {
    const el = document.createElement("div")
    el.setAttribute("id", "file-comments-" + index)

    editor.addLineWidget(Number.parseInt(line) - 1, el, { coverGutter: true })

    ReactDOM.render(<Comments comments={comments}/>, el)
  })
}

interface ReviewPageProps {
  projectId: string
}

export const ReviewPage = observer(
  function ReviewPage() {
    const { rootContent } = useReviewPageStore()

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
            <Card>

            </Card>
          </Col>

          <Col span={15}>
            <CodeMirror
              className={classes.codemirror}
              value={fixtureFile}
              options={defaultOptions}
              editorDidMount={editorDidMountHandler}
              onBeforeChange={() => undefined}
            />
          </Col>
        </Row>
      </div>
    )
  }
)
