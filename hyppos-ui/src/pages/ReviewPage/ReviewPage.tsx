import React from "react";
import { EditorConfiguration } from "codemirror"
import { Controlled as CodeMirror, ICodeMirror } from "react-codemirror2"

import "codemirror/lib/codemirror.css"
import "codemirror/mode/rust/rust";


const defaultOptions: EditorConfiguration = {
  lineNumbers: true,
  mode: {
    name: "rust"
  },
  viewportMargin: Infinity
}

const someValue = `
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

use crate::models;

pub fn find_comment_by_uid(
    uid: String,
    conn: &PgConnection,
) -> Result<Option<models::Comment>, diesel::result::Error> {
    use crate::schema::comments::dsl::*;

    let user = comments
        .filter(id.eq(uid.to_string()))
        .first::<models::Comment>(conn)
        .optional()?;

    Ok(user)
}

pub fn insert_new_comment(
    pid: &Option<String>,
    msg: &str,
    conn: &PgConnection,
) -> Result<models::Comment, diesel::result::Error> {
    use crate::schema::comments::dsl::*;

    let new_comment = models::Comment {
        id: Uuid::new_v4().to_string(),
        parent_id: pid.to_owned(),
        message: msg.to_owned(),
        created_at: Utc::now().to_owned(),
        is_deleted: false,
    };

    diesel::insert_into(comments).values(&new_comment).execute(conn)?;

    Ok(new_comment)
}
  `

export const ReviewPage = React.memo(
  function ReviewPage() {
    return (
      <CodeMirror
        value={someValue}
        options={defaultOptions}
        onBeforeChange={() => undefined}
      />
    )
  }
)
