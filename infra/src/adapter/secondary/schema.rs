table! {
    tasks (id) {
        id -> Text,
        name -> Nullable<Text>,
        command -> Text,
        env -> Nullable<Text>,
        status -> Text,
        status_log -> Nullable<Text>,
    }
}