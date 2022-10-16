mod models;

mod insert;
use self::insert::*;

mod select;
use self::select::*;

mod convenience;
use self::convenience::*;

use self::models::*;
mod schema;
use diesel::prelude::*;

fn main() {
    println!("Hello, world!");
    // // Create tasks

    // generate_data();

    // select_task();

    convenience::print_incomplete_tasks_and_todos();
}

pub fn establish_connection() -> SqliteConnection {
    match SqliteConnection::establish("./database.db") {
        Ok(n) => return n,
        Err(n) => panic!("{:?}", n),
    }
}

pub fn generate_data() {
    insert_task(NewTask {
        name: String::from("Produce software"),
    });
    //
    insert_task(NewTask {
        name: String::from("Brew coffee"),
    });

    // Insert produce software
    insert_todo(NewTodo {
        name: String::from("Write code"),
        is_completed: 0,
        task_id: 1,
    });
    insert_todo(NewTodo {
        name: String::from("Compile code"),
        is_completed: 0,
        task_id: 1,
    });
    insert_todo(NewTodo {
        name: String::from("Test program"),
        is_completed: 0,
        task_id: 1,
    });

    // Insert brew coffee
    insert_todo(NewTodo {
        name: String::from("Pour water"),
        is_completed: 0,
        task_id: 2,
    });
    insert_todo(NewTodo {
        name: String::from("Pour coffee"),
        is_completed: 0,
        task_id: 2,
    });
    insert_todo(NewTodo {
        name: String::from("Turn on"),
        is_completed: 0,
        task_id: 2,
    });
}
