use diesel::{
    insert_into,
    internal::derives,
    sql_types::{Integer, Nullable},
    Associations, Identifiable, Insertable, Queryable, deserialize,
};

use crate::schema::{task, team, team_worker, todo, worker};

// Task todo

#[derive(Queryable, Debug, Identifiable, Associations, PartialEq)]
#[belongs_to(Team)]
#[diesel(table_name = task)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub team_id: i32,
}

#[derive(Queryable, Debug, Identifiable, Associations, PartialEq)]
#[belongs_to(Task)]
#[diesel(table_name = todo)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub task_id: i32,
    pub is_completed: i32,
    pub worker_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = task)]
pub struct NewTask {
    pub name: String,
    pub team_id: i32,
}
#[derive(Insertable)]
#[diesel(table_name = todo)]
pub struct NewTodo {
    pub name: String,
    pub is_completed: i32,
    pub task_id: i32,
    pub worker_id: i32,
}

// Team workers
#[derive(Queryable, Identifiable)]
#[diesel(table_name = worker)]
pub struct Worker {
    pub id: i32,
    pub name: String,
    pub current_todo: i32,
}
#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = team)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub task_id: i32,
}
#[derive(Queryable, Associations)]
#[belongs_to(Team)]
#[belongs_to(Worker)]
#[diesel(table_name = team_worker)]
pub struct TeamWorker {
    pub id: i32,
    pub worker_id: i32,
    pub team_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = worker)]
pub struct NewWorker {
    pub name: String,
    pub current_todo: i32,
}
#[derive(Insertable)]
#[diesel(table_name = team)]
pub struct NewTeam {
    pub name: String,
    pub current_task: i32,
}
#[derive(Insertable)]
#[diesel(table_name = team_worker)]
pub struct NewTeamWorker {
    pub worker_id: i32,
    pub team_id: i32,
}
