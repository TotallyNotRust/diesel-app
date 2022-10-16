use diesel::{associations::HasTable, RunQueryDsl};

use crate::{models::{NewTask, NewTodo, NewWorker, NewTeam, NewTeamWorker}, establish_connection};

pub fn insert_task(value: NewTask) {
    use crate::schema::task::dsl::*;
    let mut _connection = &mut establish_connection();

    match diesel::insert_into(task::table())
        .values(value)
        .execute(_connection)
    {
        Ok(n) => println!("Sucessfully inserted; affected {:?} rows", n),
        Err(n) => println!("Error occured, got: {:?}", n),
    }

    return;
}
pub fn insert_todo(value: NewTodo) {
    use crate::schema::todo::dsl::*;
    let mut _connection = &mut establish_connection();

    match diesel::insert_into(todo::table())
        .values(value)
        .execute(_connection)
    {
        Ok(n) => println!("Sucessfully inserted; affected {:?} rows", n),
        Err(n) => println!("Error occured, got: {:?}", n),
    }

    return;
}

pub fn insert_worker(value: NewWorker) {
    use crate::schema::worker::dsl::*;
    let mut _connection = &mut establish_connection();

    match diesel::insert_into(worker::table())
        .values(value)
        .execute(_connection)
    {
        Ok(n) => println!("Sucessfully inserted; affected {:?} rows", n),
        Err(n) => println!("Error occured, got: {:?}", n),
    }

    return;
}
pub fn insert_team(value: NewTeam) {
    use crate::schema::team::dsl::*;
    let mut _connection = &mut establish_connection();

    match diesel::insert_into(team::table())
        .values(value)
        .execute(_connection)
    {
        Ok(n) => println!("Sucessfully inserted; affected {:?} rows", n),
        Err(n) => println!("Error occured, got: {:?}", n),
    }

    return;
}

pub fn insert_team_worker(value: NewTeamWorker) {
    use crate::schema::team_worker::dsl::*;
    let mut _connection = &mut establish_connection();

    match diesel::insert_into(team_worker::table())
        .values(value)
        .execute(_connection)
    {
        Ok(n) => println!("Sucessfully inserted; affected {:?} rows", n),
        Err(n) => println!("Error occured, got: {:?}", n),
    }

    return;
}