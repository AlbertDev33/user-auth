use diesel::table;

table! {
    users (id) {
        id -> Uuid,
        email -> VarChar,
        hash -> VarChar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Timestamptz,
    }
}

table! {
    invitations (id) {
        id -> Uuid,
        email -> VarChar,
        expires_at -> Timestamptz,
    }
}