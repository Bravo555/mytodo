use mytodo::db::{self, DbError};
use std::env;

fn main() {
    env::set_var("DATABASE_URL", "./testdb.sqlite3");

    let mut args = env::args().skip(1);

    let subcommand: &str = &args.next().unwrap();
    match subcommand {
        "new" => {
            let title: &str = &args.next().unwrap_or_else(|| {
                help();
                panic!();
            });
            new_task(title);
        }
        "show" => {
            let id = args.next();
            match id {
                Some(id) => show_task(id.parse().expect("bad id")),
                None => show_tasks(),
            }
        }
        "done" => {
            let id = args.next().expect("expected id").parse().expect("bad id");
            complete_task(id);
        }
        "delete" => {
            let id = args.next().expect("expected id").parse().expect("bad id");
            delete_task(id);
        }
        _ => help(),
    }
}

fn help() {
    println!("subcommands:");
    println!("\tnew <title>: create a new task");
}

fn new_task(title: &str) {
    let conn = db::establish_connection();
    db::create_task(&conn, &title);
    show_tasks();
}

fn show_tasks() {
    let conn = db::establish_connection();
    println!("TASKS\n-----\nid\ttitle\tdone");
    for task in db::query_task(&conn) {
        print_task(&task);
    }
}

fn show_task(id: i32) {
    let conn = db::establish_connection();
    match db::get_task_by_id(&conn, id) {
        Some(task) => print_task(&task),
        None => println!("Such task doesn't exist"),
    }
}

fn complete_task(id: i32) {
    let conn = db::establish_connection();
    match db::complete_task(&conn, id) {
        Ok(_) => println!("Task with id {} completed!", id),
        Err(DbError::NoSuchTask) => println!("Task with id {} doesn't exist", id),
    }
    show_tasks();
}

fn delete_task(id: i32) {
    let conn = db::establish_connection();
    db::delete_task(&conn, id);
    show_tasks();
}

fn print_task(task: &db::models::Task) {
    println!("{}\t{}\t{}", task.id, task.title, tick_if_done(task.done));
}

fn tick_if_done(done: bool) -> String {
    if done {
        String::from("[X]")
    } else {
        String::from("[ ]")
    }
}
