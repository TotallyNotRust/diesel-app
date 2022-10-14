use crate::{
    establish_connection,
    models::{Task, Todo},
};
use diesel::RunQueryDsl;

pub fn select_task() -> Result<Vec<Task>, ()> {
    use crate::schema::task::dsl::*;

    let _connection = &mut establish_connection();

    let tasks = match task.load::<Task>(_connection) {
        Ok(n) => n,
        Err(n) => {
            println!("An error occured: {:?}", n);
            return Err(());
        }
    };

    println!("{:?}", tasks);

    return Ok(tasks);
}

pub fn select_todo() -> Result<Vec<Todo>, ()>{
    use crate::schema::todo::dsl::*;

    let _connection = &mut establish_connection();

    let todos = match todo.load::<Todo>(_connection) {
        Ok(n) => n,
        Err(n) => {
            println!("An error occured: {:?}", n);
            return Err(());
        }
    };

    println!("{:?}", todos);

    return Ok(todos);
}
