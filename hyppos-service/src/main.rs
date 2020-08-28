extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod comments;
mod schema;
mod models;

use diesel::prelude::*;

use self::comments::*;

fn main() {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();

    let n = insert_new_comment(&None, "Hello", &conn).unwrap();

    println!("Created comment with ID {}", n.id);

    let c = find_comment_by_uid(n.id, &conn).unwrap();
    match c {
        Some(com) => println!("Created comment with text {}", com.message),
        _ => panic!("No such comment"),
    }
}
