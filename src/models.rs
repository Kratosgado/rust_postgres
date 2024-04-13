// use serde::{Deserialize, Serialize};
// use tokio_pg_mapper_derive::PostgresMapper;
// #[derive(Deserialize, PostgresMapper, Serialize)]
// #[pg_mapper(table = "users")]
// pub struct User {
//     pub email: String,
//     pub first_name: String,
//     pub last_name: String,
//     pub username: String,
// }

use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
