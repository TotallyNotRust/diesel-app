use crate::{
    establish_connection,
    insert::*,
    models::{Task, Team, Todo},
    schema::todo::is_completed,
};
use diesel::{prelude::*, sql_types::Nullable};

pub fn print_team_and_tasks() {
    let _connection = &mut establish_connection();

    use crate::models::{Task, Team};
    use crate::schema::task;
    use crate::schema::team::dsl::team;

    let teams = team
        .load::<Team>(_connection)
        .expect("Could not load teams");

    if teams.is_empty() {
        println!("No teams found");
    }

    'teamloop: for _team in teams {
        let tasks = Task::belonging_to(&_team)
            .load::<Task>(_connection)
            .expect("Could not load tasks");

        if tasks.is_empty() {
            println!("{}", _team.name);
            println!("-> Empty");

            continue 'teamloop;
        }

        println!("{} has tasks", _team.name);
        for _task in tasks {
            println!("-> {}", _task.name);
        }
    }
}

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
        team_id: 1,
    });
    //
    insert_task(NewTask {
        name: String::from("Brew coffee"),
        team_id: 3,
    });

    // Insert produce software
    insert_todo(NewTodo {
        name: String::from("Write code"),
        is_completed: 0,
        task_id: 1,
        worker_id: -1,
    });
    insert_todo(NewTodo {
        name: String::from("Compile code"),
        is_completed: 0,
        task_id: 1,
        worker_id: -1,
    });
    insert_todo(NewTodo {
        name: String::from("Test program"),
        is_completed: 0,
        task_id: 1,
        worker_id: -1,
    });

    // Insert brew coffee
    insert_todo(NewTodo {
        name: String::from("Pour water"),
        is_completed: 0,
        task_id: 2,
        worker_id: -1,
    });
    insert_todo(NewTodo {
        name: String::from("Pour coffee"),
        is_completed: 0,
        task_id: 2,
        worker_id: -1,
    });
    insert_todo(NewTodo {
        name: String::from("Turn on"),
        is_completed: 0,
        task_id: 2,
        worker_id: -1,
    });
}

pub fn seed_workers() {
    // insert teams
    {
        use crate::models::NewTeam;

        insert_team(NewTeam {
            name: String::from("Frontend"),
            current_task: 1,
        });
        insert_team(NewTeam {
            name: String::from("Backend"),
            current_task: 1,
        });
        insert_team(NewTeam {
            name: String::from("Testere"),
            current_task: 2,
        });
    }
    // insert workers
    {
        use crate::models::NewWorker;

        insert_worker(NewWorker {
            name: String::from("Steen Secher"),
            current_todo: 1,
        });
        insert_worker(NewWorker {
            name: String::from("Ejvind Møller"),
            current_todo: 2,
        });
        insert_worker(NewWorker {
            name: String::from("Konrad Sommer"),
            current_todo: 3,
        });
        insert_worker(NewWorker {
            name: String::from("Sofus Lotus"),
            current_todo: -1,
        });
        insert_worker(NewWorker {
            name: String::from("Remo Lademann"),
            current_todo: -1,
        });
        insert_worker(NewWorker {
            name: String::from("Ella Fanth"),
            current_todo: 4,
        });
        insert_worker(NewWorker {
            name: String::from("Anne Dam"),
            current_todo: 5,
        });
    }
    // insert team workers
    {
        use crate::models::NewTeamWorker;
        // Steen Secher (0)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 0,
                team_id: 0,
            });
            insert_team_worker(NewTeamWorker {
                worker_id: 0,
                team_id: 2,
            });
        }
        // Ejvind Møller (1)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 1,
                team_id: 0,
            });
        }
        // Konrad Sommer (2)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 2,
                team_id: 0,
            });
            insert_team_worker(NewTeamWorker {
                worker_id: 2,
                team_id: 1,
            });
        }
        // Sofus Lotus (3)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 3,
                team_id: 1,
            });
        }
        // Remo Lademann (4)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 4,
                team_id: 1,
            });
        }
        // Ella Fanth (5)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 5,
                team_id: 2,
            });
        }
        // Anne Damn (6)
        {
            insert_team_worker(NewTeamWorker {
                worker_id: 6,
                team_id: 2,
            });
        }
    }
}

pub fn print_teams_without_tasks() -> Vec<Team> {
    let _connection = &mut establish_connection();

    use crate::models::{Task, Team};
    use crate::schema::task;
    use crate::schema::team::dsl::team;

    let teams = team
        .load::<Team>(_connection)
        .expect("Could not load teams");

    if teams.is_empty() {
        println!("No teams found");
    }

    let mut teams_without_tasks = vec![];

    'teamloop: for _team in teams {
        let tasks = Task::belonging_to(&_team)
            .load::<Task>(_connection)
            .expect("Could not load tasks");

        if tasks.is_empty() {
            teams_without_tasks.push(_team);
        }
    }
    return teams_without_tasks;
}

pub fn print_progress() {
    let _connection = &mut establish_connection();

    use crate::models::{Task, Team};
    use crate::schema::task::dsl::task;
    use crate::schema::team::dsl::team;
    use crate::schema::todo::dsl::{id, todo};

    let join = team.left_outer_join(task);

    let team_tasks = join.load::<(Team, Option<Task>)>(_connection).expect("");

    for (_team, _option_task) in team_tasks {
        let _task;
        match _option_task {
            Some(n) => {
                println!("Found task for {}: {}", _team.name, n.name);
                _task = n;
            }
            None => {
                println!("No task for {}", _team.name);
                continue;
            }
        }
        let todos = Todo::belonging_to(&_task)
            .select(is_completed)
            .load::<i32>(_connection)
            .expect("Could not load todos");
        let mut amount_completed = 0.0;
        let mut amount_total = 0.0;

        for completed in todos {
            if completed == 1 {
                amount_completed += 1.0;
            }
            amount_total += 1.0;
        }

        println!(
            "{} is {:.1}% completed ({}/{} tasks completed)",
            _team.name,
            amount_completed / amount_total * 100.0,
            amount_completed,
            amount_total,
        )
    }
}
