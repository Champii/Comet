// @generated automatically by Diesel CLI.

crate::diesel::table! {
    channels (id) {
        id -> Int4,
        name -> Text,
    }
}

crate::diesel::table! {
    messages (id) {
        id -> Int4,
        channel_id -> Int4,
        sender -> Text,
        content -> Text,
    }
}

crate::diesel::allow_tables_to_appear_in_same_query!(
    channels,
    messages,
);
