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

    // convenience::print_incomplete_tasks_and_todos();

    // convenience::seed_workers();

    // print_team_and_tasks();

    // println!("{:?}", print_teams_without_tasks());

    print_progress()
}

pub fn establish_connection() -> SqliteConnection {
    match SqliteConnection::establish("./database.db") {
        Ok(n) => return n,
        Err(n) => panic!("{:?}", n),
    }
}
