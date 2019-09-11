#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use mytodo::db;

#[get("/tasks")]
fn tasks_get() -> String {
    let mut response: Vec<_> = Vec::new();
    let conn = db::establish_connection();
    for task in db::query_task(&conn) {
        response.push(task.title);
    }
    response.join("\n")
}

fn main() {
    rocket::ignite().mount("/", routes![tasks_get]).launch();
}
