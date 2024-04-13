use rust_postgres::*;
use diesel::prelude::*;
use std::env::args;

fn main(){
    use self::schema::posts::dsl::*;

    let target = args()
        .nth(1)
        .expect("delete_post requires a post id");

    let pattern = format!("%{}%", target);
    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}