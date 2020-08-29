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


export const ReviewPage = React.memo(
  function ReviewPage() {
    return (
      <div className={classes.root}>
        <CodeMirror
          className={classes.codemirror}
          value={fixtureFile}
          options={defaultOptions}
          editorDidMount={editorDidMountHandler}
          onBeforeChange={() => undefined}
        />
      </div>
    )
  }
)
