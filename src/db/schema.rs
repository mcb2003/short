table! {
    links (id) {
        id -> Uuid,
        slug -> Nullable<Text>,
        uri -> Text,
        description -> Text,
        deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
