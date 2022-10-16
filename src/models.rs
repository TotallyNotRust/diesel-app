use diesel::{
    insert_into,
    sql_types::{Integer, Nullable},
    Associations, Identifiable, Insertable, Queryable,
};

use crate::schema::{
    task,
    todo::{self},
};

#[derive(Queryable, Debug, Identifiable, PartialEq)]
#[diesel(table_name = task)]
pub struct Task {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Debug, Identifiable, Associations, PartialEq)]
#[belongs_to(Task)]
#[diesel(table_name = todo)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub task_id: i32,
    pub is_completed: i32,
}

#[derive(Insertable)]
#[diesel(table_name = task)]
pub struct NewTask {
    pub name: String,
}
#[derive(Insertable)]
#[diesel(table_name = todo)]
pub struct NewTodo {
    pub name: String,
    pub is_completed: i32,
    pub task_id: i32,
}
