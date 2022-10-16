use crate::{establish_connection, models::Task};
use diesel::{BelongingToDsl, RunQueryDsl};

pub fn print_incomplete_tasks_and_todos() {
    let _connection = &mut establish_connection();

    use crate::models::Todo;
    use crate::schema::task::dsl::task;

    // load all tasks
    let tasks = task.load::<Task>(_connection).expect("");

    'taskloop: for _task in tasks {
        // Hent alle todos med foreign key til en task
        let todos = Todo::belonging_to(&_task).load::<Todo>(_connection);
        // Check om vi kunne hente todos succesfuldt
        match todos {
            Err(n) => {
                println!("Could not get tasks for {:?}, got error: {:?}", _task, n);
                continue 'taskloop;
            }
            Ok(n) => {
                // Vis vi har hentet todo succesfuldt så tjek om der er nogen der ikke er "completed"
                let mut unfinished: Vec<Todo> = vec![];
                for _todo in n {
                    if _todo.is_completed != 1 {
                        unfinished.push(_todo);
                    }
                }
                // Hvis der er nogen todos der ikke er færdige så print dem.
                if !(unfinished.is_empty()) {
                    println!("{} has unfinished tasks", _task.name);
                    for _todo in unfinished {
                        println!("-> {}", _todo.name);
                    }
                }
            }
        }
    }
}
