// @generated automatically by Diesel CLI.

diesel::table! {
    task (id) {
        id -> Integer,
        name -> Text,
        team_id -> Integer,
    }
}

diesel::table! {
    team (id) {
        id -> Integer,
        name -> Text,
        current_task -> Integer,
    }
}

diesel::table! {
    team_worker (id) {
        id -> Integer,
        worker_id -> Integer,
        team_id -> Integer,
    }
}

diesel::table! {
    todo (id) {
        id -> Integer,
        name -> Text,
        is_completed -> Integer,
        task_id -> Integer,
        worker_id -> Integer,
    }
}

diesel::table! {
    worker (id) {
        id -> Integer,
        name -> Text,
        current_todo -> Integer,
    }
}

diesel::joinable!(team_worker -> worker (worker_id));

diesel::allow_tables_to_appear_in_same_query!(
    task,
    team,
    team_worker,
    todo,
    worker,
);
