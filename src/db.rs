use diesel::{prelude::*, sqlite::SqliteConnection};
use std::env;

mod models;
mod schema;

pub fn establish_connection() -> SqliteConnection {
        let db = env::var("DATABASE_URL").expect("DATABASE_URL needs to be provided");
        SqliteConnection::establish(&db).expect(&format!("Error connecting to {}", db))
}

pub fn create_task<'a>(connection: &SqliteConnection, title: &'a str) {
        let task = models::NewTask { title, done: false };

        diesel::insert_into(schema::task::table)
                .values(&task)
                .execute(connection)
                .expect("Error inserting new task");
}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
        schema::task::table
                .load::<models::Task>(&*connection)
                .expect("Error loading tasks")
}
