table! {
    events (id) {
        id -> Integer,
        uid -> Text,
        content -> Text,
        finished -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
