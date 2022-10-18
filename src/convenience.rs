use crate::{establish_connection, insert::*, models::Task};
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
pub fn generate_data() {
    use crate::insert::*;
    use crate::models::*;

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

pub fn seed_workers() {
    // insert teams
    {
        use crate::models::NewTeam;

        insert_team(NewTeam {
            name: String::from("Frontend"),
        });
        insert_team(NewTeam {
            name: String::from("Backend"),
        });
        insert_team(NewTeam {
            name: String::from("Testere"),
        });
    }
    // insert workers
    {
        use crate::models::NewWorker;

        insert_worker(NewWorker {
            name: String::from("Steen Secher"),
        });
        insert_worker(NewWorker {
            name: String::from("Ejvind Møller"),
        });
        insert_worker(NewWorker {
            name: String::from("Konrad Sommer"),
        });
        insert_worker(NewWorker {
            name: String::from("Sofus Lotus"),
        });
        insert_worker(NewWorker {
            name: String::from("Remo Lademann"),
        });
        insert_worker(NewWorker {
            name: String::from("Ella Fanth"),
        });
        insert_worker(NewWorker {
            name: String::from("Anne Dam"),
        });
    }
    // insert team workers
    {
        use crate::models::NewTeamWorker;
        // Steen Secher (1)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 1,
                team_id: 1,
            });
            insert_team_worker(NewTeamWorker {
                worker_id: 1,
                team_id: 3,
            });
        }
        // Ejvind Møller (2)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 2,
                team_id: 1,
            });
        }
        // Konrad Sommer (3)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 3,
                team_id: 1,
            });
            insert_team_worker(NewTeamWorker {
                worker_id: 3,
                team_id: 2,
            });
        }
        // Sofus Lotus (4)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 4,
                team_id: 2,
            });
        }
        // Remo Lademann (5)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 5,
                team_id: 2,
            });
        }
        // Ella Fanth (6)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 6,
                team_id: 3,
            });
        }
        // Anne Damn (7)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 7,
                team_id: 3,
            });
        }
    }
}
