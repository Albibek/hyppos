use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::comments::schema::comments;


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Comment {
    pub id: uuid::Uuid,
    pub parent_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub project_id: uuid::Uuid,
    pub hash: Option<String>,
    pub file_id: uuid::Uuid,
    pub line_no: Option<i64>,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub is_deleted: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewComment {
    pub parent_id: Option<uuid::Uuid>,
    pub user_id: uuid::Uuid,
    pub project_id: uuid::Uuid,
    pub hash: Option<String>,
    pub file_id: uuid::Uuid,
    pub line_no: Option<i64>,
    pub message: String,
}