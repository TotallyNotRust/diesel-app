use std::collections::HashMap;

use crate::{
    establish_connection,
    models::{Task, Todo},
};
use diesel::{associations::HasTable, QueryDsl, RunQueryDsl};
use diesel::{expression_methods::ExpressionMethods, Identifiable};

pub fn print_incomplete_tasks_and_todos() {
    let _connection = &mut establish_connection();

    {
        use crate::schema::task::dsl::*;
        use crate::schema::todo::dsl::{is_completed, todo};

        let values: Vec<(Task, Todo)> = task
            .inner_join(todo::table())
            .filter(is_completed.eq(0))
            .load::<(Task, Todo)>(_connection)
            .expect("");

        println!("{:?}", values);

        let mut map: HashMap<i32, i32> = HashMap::new();
        for value in values {
            if value.1.is_completed != 1 {
                let amount = map.remove(value.0.id());
                match amount {
                    Some(n) => {
                        map.remove(value.0.id());
                        map.insert(value.0.id, 1 + n);
                    }
                    None => {
                        map.insert(value.0.id().to_owned(), 1);
                    }
                }
            }
        }
        println!("{:?}", map);
        // let todos = match task.load::<Task>(_connection) {
        //     Ok(n) => n,
        //     Err(n) => {
        //         println!("An error occured: {:?}", n);
        //         return;
        //     }
        // };
    }
}
