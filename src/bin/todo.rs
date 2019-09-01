use mytodo::db;
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
            show_tasks();
        }
        "done" => {
            let id = args.next().expect("expected id");
            mark_task_done(id);
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
}

fn show_tasks() {
    let conn = db::establish_connection();
    println!("TASKS\n-----");
    for task in db::query_task(&conn) {
        println!("{}: done - {}", task.title, task.done);
    }
}

fn mark_task_done(id: i32) {}
