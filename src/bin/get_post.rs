use self::models::Post;
use diesel::prelude::*;
use rust_postgres::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::posts;

    let post_id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional(); // this allows for returning a Result<Option<Post>> otherwise it would return a Result<Post>

    match post {
        Ok(Some(post)) => {
            println!("Found post {}", post.title);
        }
        Ok(None) => {
            println!("Unable to find post {}", post_id);
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}
