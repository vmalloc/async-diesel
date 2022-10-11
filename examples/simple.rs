use async_diesel2::*;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use std::error::Error;
use uuid::Uuid;

// Schema
table! {
    users (id) {
        id -> Uuid,
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect
    let manager =
        ConnectionManager::<PgConnection>::new("postgres://postgres@localhost/async_diesel__test");
    let pool = Pool::builder().build(manager)?;

    // Add
    println!("add a user");
    diesel::insert_into(users::table)
        .values(users::id.eq(Uuid::new_v4()))
        .execute_async(&pool)
        .await?;

    // Count
    let num_users: i64 = users::table.count().get_result_async(&pool).await?;
    println!("now there are {:?} users", num_users);

    Ok(())
}
