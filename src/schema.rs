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
        todoId -> Integer,
    }
}

diesel::joinable!(todo -> task (todoId));

diesel::allow_tables_to_appear_in_same_query!(
    task,
    todo,
);
