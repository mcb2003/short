table! {
    links (id) {
        id -> Uuid,
        slug -> Nullable<Text>,
        uri -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
