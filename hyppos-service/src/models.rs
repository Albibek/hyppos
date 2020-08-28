use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::schema::comments;


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Comment {
    pub id: uuid::Uuid,

    pub parent_id: uuid::Uuid,
    pub message: String,

    pub user_id: uuid::Uuid,
    pub project_id: uuid::Uuid,
    pub hash: Option<String>,
    pub file_id: uuid::Uuid,
    pub line_no: Option<i64>,

    pub is_deleted: bool,

    pub created_at: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewComment {
    pub parent_id: Option<uuid::Uuid>,
    pub message: String,

    pub user_id: uuid::Uuid,
    pub project_id: uuid::Uuid,
    pub hash: Option<String>,
    pub file_id: uuid::Uuid,
    pub line_no: Option<i64>,
}