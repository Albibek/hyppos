use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::schema::comments;


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Comment {
    pub id: String,
    pub parent_id: Option<String>,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub is_deleted: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewComment {
    pub parent_id: Option<String>,
    pub message: String,
}