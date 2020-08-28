extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use uuid::Uuid;

mod comments;
mod models;
mod schema;

use diesel::prelude::*;

use self::comments::*;

fn main() {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();

    let first = insert_new_comment(
        &models::NewComment {
            parent_id: None,
            message: String::from("Level 1"),
            user_id: uuid::Uuid::new_v4(),
            project_id: uuid::Uuid::new_v4(),
            hash: None,
            file_id: uuid::Uuid::new_v4(),
            line_no: None,
        },
        &conn,
    )
    .unwrap();

    println!("Created comment with ID {}", first.id);

    let _ = insert_new_comment(
        &models::NewComment {
            parent_id: Some(first.id),
            message: String::from("Level 2"),
            user_id: uuid::Uuid::new_v4(),
            project_id: uuid::Uuid::new_v4(),
            hash: None,
            file_id: uuid::Uuid::new_v4(),
            line_no: None,
        },
        &conn,
    )
    .unwrap();

    let c = find_comment_by_id(first.id, &conn).unwrap();
    match c {
        Some(com) => println!("Created comment with text {}", com.message),
        _ => panic!("No such comment"),
    }
}
