// @generated automatically by Diesel CLI.

crate::diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Text,
        completed -> Bool,
    }
}
