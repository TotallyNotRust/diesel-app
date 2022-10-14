use diesel::{insert_into, sql_types::Integer, Insertable, Queryable};

use crate::schema::{task, todo};

#[derive(Queryable, Debug)]
#[diesel(table_name = task)]
pub struct Task {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Debug)]
#[diesel(table_name = todo)]

pub struct Todo {
    pub id: i32,
    pub name: String,
    pub todo_id: i32,
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
    pub todo_id: i32,
}
