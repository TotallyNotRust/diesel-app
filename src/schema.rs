// @generated automatically by Diesel CLI.

diesel::table! {
    task (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    team (id) {
        id -> Integer,
        name -> Text,
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
        task_id -> Integer,
        is_completed -> Integer,
    }
}

diesel::table! {
    worker (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(team_worker -> team (team_id));
diesel::joinable!(team_worker -> worker (worker_id));
diesel::joinable!(todo -> task (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    task,
    team,
    team_worker,
    todo,
    worker,
);
