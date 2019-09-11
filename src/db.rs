pub mod models;
mod schema;

use diesel::{prelude::*, sqlite::SqliteConnection};
use schema::task;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let db = env::var("DATABASE_URL").expect("DATABASE_URL needs to be provided");
    SqliteConnection::establish(&db).expect(&format!("Error connecting to {}", db))
}

pub fn create_task<'a>(connection: &SqliteConnection, title: &'a str) {
    let task = models::NewTask { title, done: false };

    diesel::insert_into(task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    task::table
        .load::<models::Task>(&*connection)
        .expect("Error loading tasks")
}

pub fn complete_task(connection: &SqliteConnection, id: i32) -> Result<(), DbError> {
    let result = diesel::update(task::table.find(id))
        .set(task::done.eq(true))
        .execute(connection);
    match result {
        Ok(1) => Ok(()),
        _ => Err(DbError::NoSuchTask),
    }
}

pub fn get_task_by_id(connection: &SqliteConnection, id: i32) -> Option<models::Task> {
    task::table
        .find(id)
        .load::<models::Task>(&*connection)
        .expect("error getting task")
        .pop()
}

pub fn delete_task(connection: &SqliteConnection, id: i32) {
    diesel::delete(task::table.find(id))
        .execute(connection)
        .expect("error when deleting");
}

pub enum DbError {
    NoSuchTask,
}
