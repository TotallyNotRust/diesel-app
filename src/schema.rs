// @generated automatically by Diesel CLI.

diesel::table! {
    task (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    todo (id) {
        id -> Integer,
        name -> Text,
        todo_id -> Integer,
    }
}

diesel::joinable!(todo -> task (todo_id));

diesel::allow_tables_to_appear_in_same_query!(
    task,
    todo,
);
