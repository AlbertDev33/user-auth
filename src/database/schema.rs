use diesel::table;

table! {
    users (email) {
        email -> VarChar,
        hash -> VarChar,
        created_at -> Timestamp,
    }
}

table! {
    invitations (id) {
        id -> Uuid,
        email -> VarChar,
        expires_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(invitations, users);