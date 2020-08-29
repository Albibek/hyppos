use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::schema::{comments, projects, users};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: uuid::Uuid,
    pub external_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Project {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub external_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProjectWithID {
    pub external_id: i64,
    pub user_id: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProject {
    pub external_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Comment {
    pub id: uuid::Uuid,
    pub parent_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub project_id: uuid::Uuid,
    pub commit_id: String,
    pub file_id: String,
    pub line_no: Option<i64>,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub is_deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewComment {
    pub parent_id: Option<uuid::Uuid>,
    pub project_id: uuid::Uuid,
    pub commit_id: String,
    pub file_id: String,
    pub line_no: Option<i64>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCommentWithID {
    pub parent_id: Option<uuid::Uuid>,
    pub user_id: uuid::Uuid,
    pub project_id: uuid::Uuid,
    pub commit_id: String,
    pub file_id: String,
    pub line_no: Option<i64>,
    pub message: String,
}
