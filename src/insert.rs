use diesel::{associations::HasTable, RunQueryDsl};

use crate::{models::{NewTask, NewTodo}, establish_connection};

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